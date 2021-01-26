use alloc::sync::Arc;
use drone_stm32f4_gpio_drv::{AlternateMode, GpioPin, GpioPinMap, PinAf};
use core::marker::PhantomData;
use drone_core::{
    fib::{self, Fiber},
    reg::marker::RwReg,
};
use drone_cortexm::{reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::tim::general::{
    traits::*, GeneralTimMap, TimCcerCc2E, TimCcerCc3E, TimCcerCc4E, TimCcmr1OutputCc2S,
    TimCcmr2Output, TimCcr2, TimCcr3, TimCcr4, TimDierCc2Ie, TimDierCc3Ie, TimDierCc4Ie,
    TimSrCc2If, TimSrCc3If, TimSrCc4If,
};
use futures::{future, Stream};

use crate::traits::*;
use crate::{gen::GeneralTimDiverged, shared::DontCare};

/// The timer channel driver.
pub struct GeneralTimChDrv<Tim: GeneralTimMap, Int: IntToken, Ch, Mode> {
    pub(crate) tim: Arc<GeneralTimDiverged<Tim>>,
    pub(crate) tim_int: Int,
    ch: PhantomData<Ch>,
    mode: Mode,
}

impl<Tim: GeneralTimMap, Int: IntToken, Ch, Mode> GeneralTimChDrv<Tim, Int, Ch, Mode> {
    /// Create a new timer channel driver.
    pub(crate) fn new(tim: Arc<GeneralTimDiverged<Tim>>, tim_int: Int, mode: Mode) -> Self {
        Self {
            tim,
            tim_int,
            ch: PhantomData,
            mode,
        }
    }
}

impl<Tim: GeneralTimMap, Int: IntToken, Ch: GeneralTimCh<Tim>>
    GeneralTimChDrv<Tim, Int, Ch, DontCare>
{
    /// Configure the channel as Output/Compare.
    pub fn into_output_compare(self) -> GeneralTimChDrv<Tim, Int, Ch, OutputCompareMode> {
        Ch::configure_output(&self.tim);
        GeneralTimChDrv::new(self.tim, self.tim_int, OutputCompareMode)
    }

    /// Configure the channel as Input/Capture.
    pub fn into_input_capture<Pin: GpioPinMap, Af: PinAf, PinType, PinPull, Sel: InputSelection>(
        self,
        pin: GpioPin<Pin, AlternateMode<Af>, PinType, PinPull>,
        sel: Sel,
    ) -> GeneralTimChDrv<Tim, Int, Ch, InputCaptureMode<Pin, Af, PinType, PinPull, Sel>> {
        Ch::configure_input(&self.tim, sel);
        GeneralTimChDrv::new(self.tim, self.tim_int, InputCaptureMode { pin, sel: PhantomData })
    }
}

impl<Tim: GeneralTimMap, Int: IntToken, Ch: GeneralTimCh<Tim>> TimerCompareCh
    for GeneralTimChDrv<Tim, Int, Ch, OutputCompareMode>
{
    type Stop = Self;

    fn next(&mut self, compare: u32, soon: bool) -> TimerCompareNext<'_, Self::Stop> {
        let tim_sr = self.tim.tim_sr;
        let tim_dier = self.tim.tim_dier;
        let timeout_future = Box::pin(self.tim_int.add_future(fib::new_fn(move || {
            if Ch::get_ccif(tim_sr) {
                Ch::clear_ccie(tim_dier);
                fib::Complete(())
            } else {
                fib::Yielded(())
            }
        })));

        use drone_core::token::Token;
        let tim_ch_ccr = unsafe { Ch::CTimCcr::take() };
        Ch::set_ccr(tim_ch_ccr, compare);
        Ch::clear_ccif(tim_sr);
        Ch::set_ccie(tim_dier);

        let already_passed = if soon {
            // Sample counter after interrupt is setup.
            let counter = self.tim.tim_cnt.cnt().read_bits() as u32;

            // Let's see if counter is later than compare in which case the time has already elapsed
            let max = self.tim.tim_arr.arr().read_bits() as u32;
            let half_period = max / 2; // equivalent to (max + 1) / 2 as we assume max to be odd.
            counter.wrapping_sub(compare) > half_period
        } else {
            false
        };

        if already_passed {
            // The counter has already passed the comfigured compare value - skip interrupt
            Ch::clear_ccie(tim_dier);
            drop(timeout_future);

            TimerCompareNext::new(self, Box::pin(future::ready(())))
        } else {
            TimerCompareNext::new(self, Box::pin(timeout_future))
        }
    }
}

impl<Tim: GeneralTimMap, Int: IntToken, Ch: GeneralTimCh<Tim> + Send> TimerCompareStop
    for GeneralTimChDrv<Tim, Int, Ch, OutputCompareMode>
{
    fn stop(&mut self) {
        Ch::clear_ccie(self.tim.tim_dier);
    }
}

pub trait IntoPinInputCaptureMode<
    Tim: GeneralTimMap,
    Int: IntToken,
    Ch: GeneralTimCh<Tim>,
    Sel,
    Pin: drone_stm32_map::periph::gpio::pin::GpioPinMap,
    Af: drone_stm32f4_gpio_drv::PinAf,
    PinType,
    PinPull,
>
{
    /// Configure the channel as Input/Capture from a specific GPIO pin.
    fn into_input_capture_pin(
        self,
        pin: drone_stm32f4_gpio_drv::GpioPin<
            Pin,
            drone_stm32f4_gpio_drv::AlternateMode<Af>,
            PinType,
            PinPull,
        >,
    ) -> GeneralTimChDrv<Tim, Int, Ch, InputCaptureMode<Pin, Af, PinType, PinPull, Sel>>;
}

#[macro_export]
macro_rules! general_tim_channel {
    ($($tim_ch:ident<$tim:ident>, $pin:ident<$pin_af:ident> -> $sel:ident;)+) => {
        $(
            impl<Int: drone_cortexm::thr::IntToken, PinType, PinPull>
                crate::IntoPinInputCaptureMode<
                    $tim,
                    Int,
                    $tim_ch,
                    $sel,
                    $pin,
                    $pin_af,
                    PinType,
                    PinPull,
                > for GeneralTimChDrv<$tim, Int, $tim_ch, crate::shared::DontCare>
            {
                fn into_input_capture_pin(
                    self,
                    pin: drone_stm32f4_gpio_drv::GpioPin<
                        $pin,
                        drone_stm32f4_gpio_drv::AlternateMode<$pin_af>,
                        PinType,
                        PinPull,
                    >,
                ) -> GeneralTimChDrv<$tim, Int, $tim_ch, crate::InputCaptureMode<$pin, $pin_af, PinType, PinPull, $sel>> {
                    self.into_input_capture(pin, $sel)
                }
            }
        )+
    };
}

impl<Tim: GeneralTimMap, Int: IntToken, Ch: GeneralTimCh<Tim> + Send + 'static, Pin: GpioPinMap, Af: PinAf, PinType: Send, PinPull: Send, Sel: Send>
    GeneralTimChDrv<Tim, Int, Ch, InputCaptureMode<Pin, Af, PinType, PinPull, Sel>>
{
    fn capture_stream<Item>(
        &mut self,
        stream_factory: impl FnOnce(
            Int,
            Tim::CTimSr,
            Ch::CTimCcr,
        ) -> core::pin::Pin<Box<dyn Stream<Item = Item> + Send>>,
    ) -> CaptureStream<'_, Self, Item> {
        let tim_sr = self.tim.tim_sr;
        use drone_core::token::Token;
        let tim_ch_ccr = unsafe { Ch::CTimCcr::take() };
        let stream = stream_factory(self.tim_int, tim_sr, tim_ch_ccr);
        Ch::clear_ccif(tim_sr);
        Ch::set_cce(self.tim.tim_ccer);
        Ch::set_ccie(self.tim.tim_dier);
        CaptureStream::new(self, stream)
    }

    fn capture_fib<Return>(
        tim_sr: Tim::CTimSr,
        tim_ch_ccr: Ch::CTimCcr,
    ) -> impl Fiber<Input = (), Yield = Option<u32>, Return = Return> {
        fib::new_fn(move || {
            if Ch::get_ccif(tim_sr) {
                let capture = Ch::get_ccr(tim_ch_ccr);
                Ch::clear_ccif(tim_sr);
                fib::Yielded(Some(capture))
            } else {
                fib::Yielded(None)
            }
        })
    }
}

impl<
        Tim: GeneralTimMap,
        Int: IntToken,
        Ch: GeneralTimCh<Tim> + Send + 'static,
        Pin: GpioPinMap,
        Af: PinAf,
        PinType: Send + 'static,
        PinPull: Send + 'static,
        Sel: Send + 'static,
    > TimerCaptureCh for GeneralTimChDrv<Tim, Int, Ch, InputCaptureMode<Pin, Af, PinType, PinPull, Sel>>
{
    type Stop = Self;

    fn saturating_stream(&mut self, capacity: usize) -> CaptureStream<'_, Self::Stop, u32> {
        self.capture_stream(|int, tim_sr, tim_ch_ccr| {
            Box::pin(int.add_saturating_stream(capacity, Self::capture_fib(tim_sr, tim_ch_ccr)))
        })
    }

    fn overwriting_stream(&mut self, capacity: usize) -> CaptureStream<'_, Self::Stop, u32> {
        self.capture_stream(|int, tim_sr, tim_ch_ccr| {
            Box::pin(int.add_overwriting_stream(capacity, Self::capture_fib(tim_sr, tim_ch_ccr)))
        })
    }

    fn try_stream(
        &mut self,
        capacity: usize,
    ) -> CaptureStream<'_, Self::Stop, Result<u32, ChannelCaptureOverflow>> {
        self.capture_stream(|int, tim_sr, tim_ch_ccr| {
            Box::pin(int.add_try_stream(
                capacity,
                |_| Err(ChannelCaptureOverflow),
                Self::capture_fib(tim_sr, tim_ch_ccr),
            ))
        })
    }
}

impl<Tim: GeneralTimMap, Int: IntToken, Ch: GeneralTimCh<Tim> + Send, Pin: GpioPinMap + Send, Af: PinAf, PinType: Send, PinPull: Send, Sel: Send> CaptureStop
    for GeneralTimChDrv<Tim, Int, Ch, InputCaptureMode<Pin, Af, PinType, PinPull, Sel>>
{
    #[inline]
    fn get(&self) -> bool {
        self.mode.pin.get()
    }

    fn stop(&mut self) {
        Ch::clear_ccie(self.tim.tim_dier);
        Ch::clear_cce(self.tim.tim_ccer);
    }
}

pub trait GeneralTimCh<Tim: GeneralTimMap>: Send {
    type CTimCcr: RwReg<Crt> + Copy;

    fn configure_output(tim: &GeneralTimDiverged<Tim>);
    fn configure_input<Sel: InputSelection>(tim: &GeneralTimDiverged<Tim>, sel: Sel);

    fn get_cce(tim_ccer: Tim::CTimCcer) -> bool;
    fn set_cce(tim_ccer: Tim::CTimCcer);
    fn clear_cce(tim_ccer: Tim::CTimCcer);
    fn set_ccie(tim_dier: Tim::CTimDier);
    fn clear_ccie(tim_dier: Tim::CTimDier);
    fn get_ccif(tim_sr: Tim::CTimSr) -> bool;
    fn clear_ccif(tim_sr: Tim::CTimSr);
    fn get_ccr(tim_ch_ccr: Self::CTimCcr) -> u32;
    fn set_ccr(tim_ch_ccr: Self::CTimCcr, value: u32);
}

pub trait InputSelection {
    const CC_SEL: u32;
}
impl InputSelection for DirectSelection {
    const CC_SEL: u32 = 0b01;
}
impl InputSelection for IndirectSelection {
    const CC_SEL: u32 = 0b10;
}
impl InputSelection for TrcSelection {
    const CC_SEL: u32 = 0b11;
}

impl<Tim: GeneralTimMap> GeneralTimCh<Tim> for TimCh1 {
    type CTimCcr = Tim::CTimCcr1;

    fn configure_output(tim: &GeneralTimDiverged<Tim>) {
        tim.tim_ccmr1_output
            .modify_reg(|r, v| r.cc1s().write(v, 0b00));
    }

    fn configure_input<Sel: InputSelection>(tim: &GeneralTimDiverged<Tim>, _sel: Sel) {
        tim.tim_ccmr1_output
            .modify_reg(|r, v| r.cc1s().write(v, Sel::CC_SEL));
    }

    fn get_cce(tim_ccer: Tim::CTimCcer) -> bool {
        tim_ccer.cc1e().read_bit()
    }

    fn set_cce(tim_ccer: Tim::CTimCcer) {
        tim_ccer.cc1e().set_bit();
    }

    fn clear_cce(tim_ccer: Tim::CTimCcer) {
        tim_ccer.cc1e().clear_bit();
    }

    fn set_ccie(tim_dier: Tim::CTimDier) {
        tim_dier.modify_reg(|r, v| r.cc1ie().set(v));
    }

    fn clear_ccie(tim_dier: Tim::CTimDier) {
        tim_dier.modify_reg(|r, v| r.cc1ie().clear(v));
    }

    fn get_ccif(tim_sr: Tim::CTimSr) -> bool {
        tim_sr.cc1if().read_bit()
    }

    fn clear_ccif(tim_sr: Tim::CTimSr) {
        // rc_w0: Clear flag by writing a 0, 1 has no effect.
        let mut val = unsafe { Tim::STimSr::val_from(u32::MAX) };
        tim_sr.cc1if().clear(&mut val);
        tim_sr.store_val(val);
    }

    fn get_ccr(tim_ch_ccr: Self::CTimCcr) -> u32 {
        tim_ch_ccr.load_bits()
    }

    fn set_ccr(tim_ch_ccr: Self::CTimCcr, value: u32) {
        tim_ch_ccr.store_bits(value);
    }
}

impl<
        Tim: GeneralTimMap + TimCcmr1OutputCc2S + TimDierCc2Ie + TimCcerCc2E + TimSrCc2If + TimCcr2,
    > GeneralTimCh<Tim> for TimCh2
{
    type CTimCcr = Tim::CTimCcr2;

    fn configure_output(tim: &GeneralTimDiverged<Tim>) {
        tim.tim_ccmr1_output
            .modify_reg(|r, v| r.cc2s().write(v, 0b00));
    }

    fn configure_input<Sel: InputSelection>(tim: &GeneralTimDiverged<Tim>, _sel: Sel) {
        tim.tim_ccmr1_output
            .modify_reg(|r, v| r.cc2s().write(v, Sel::CC_SEL));
    }

    fn get_cce(tim_ccer: Tim::CTimCcer) -> bool {
        tim_ccer.cc2e().read_bit()
    }

    fn set_cce(tim_ccer: Tim::CTimCcer) {
        tim_ccer.cc2e().set_bit();
    }

    fn clear_cce(tim_ccer: Tim::CTimCcer) {
        tim_ccer.cc2e().clear_bit();
    }

    fn set_ccie(tim_dier: Tim::CTimDier) {
        tim_dier.modify_reg(|r, v| r.cc2ie().set(v));
    }

    fn clear_ccie(tim_dier: Tim::CTimDier) {
        tim_dier.modify_reg(|r, v| r.cc2ie().clear(v));
    }

    fn get_ccif(tim_sr: Tim::CTimSr) -> bool {
        tim_sr.cc2if().read_bit()
    }

    fn clear_ccif(tim_sr: Tim::CTimSr) {
        // rc_w0: Clear flag by writing a 0, 1 has no effect.
        let mut val = unsafe { Tim::STimSr::val_from(u32::MAX) };
        tim_sr.cc2if().clear(&mut val);
        tim_sr.store_val(val);
    }

    fn get_ccr(tim_ch_ccr: Self::CTimCcr) -> u32 {
        tim_ch_ccr.load_bits()
    }

    fn set_ccr(tim_ch_ccr: Self::CTimCcr, value: u32) {
        tim_ch_ccr.store_bits(value);
    }
}

impl<Tim: GeneralTimMap + TimCcmr2Output + TimDierCc3Ie + TimCcerCc3E + TimSrCc3If + TimCcr3>
    GeneralTimCh<Tim> for TimCh3
{
    type CTimCcr = Tim::CTimCcr3;

    fn configure_output(_tim: &GeneralTimDiverged<Tim>) {
        use drone_core::token::Token;
        let tim_ccmr2_output = unsafe { Tim::STimCcmr2Output::take() };
        tim_ccmr2_output.modify_reg(|r, v| r.cc3s().write(v, 0b00));
    }

    fn configure_input<Sel: InputSelection>(_tim: &GeneralTimDiverged<Tim>, _sel: Sel) {
        use drone_core::token::Token;
        let tim_ccmr2_output = unsafe { Tim::STimCcmr2Output::take() };
        tim_ccmr2_output.modify_reg(|r, v| r.cc3s().write(v, Sel::CC_SEL));
    }

    fn get_cce(tim_ccer: Tim::CTimCcer) -> bool {
        tim_ccer.cc3e().read_bit()
    }

    fn set_cce(tim_ccer: Tim::CTimCcer) {
        tim_ccer.cc3e().set_bit();
    }

    fn clear_cce(tim_ccer: Tim::CTimCcer) {
        tim_ccer.cc3e().clear_bit();
    }

    fn set_ccie(tim_dier: Tim::CTimDier) {
        tim_dier.modify_reg(|r, v| r.cc3ie().set(v));
    }

    fn clear_ccie(tim_dier: Tim::CTimDier) {
        tim_dier.modify_reg(|r, v| r.cc3ie().clear(v));
    }

    fn get_ccif(tim_sr: Tim::CTimSr) -> bool {
        tim_sr.cc3if().read_bit()
    }

    fn clear_ccif(tim_sr: Tim::CTimSr) {
        // rc_w0: Clear flag by writing a 0, 1 has no effect.
        let mut val = unsafe { Tim::STimSr::val_from(u32::MAX) };
        tim_sr.cc3if().clear(&mut val);
        tim_sr.store_val(val);
    }

    fn get_ccr(tim_ch_ccr: Self::CTimCcr) -> u32 {
        tim_ch_ccr.load_bits()
    }

    fn set_ccr(tim_ch_ccr: Self::CTimCcr, value: u32) {
        tim_ch_ccr.store_bits(value);
    }
}

impl<Tim: GeneralTimMap + TimCcmr2Output + TimDierCc4Ie + TimCcerCc4E + TimSrCc4If + TimCcr4>
    GeneralTimCh<Tim> for TimCh4
{
    type CTimCcr = Tim::CTimCcr4;

    fn configure_output(_tim: &GeneralTimDiverged<Tim>) {
        use drone_core::token::Token;
        let tim_ccmr2_output = unsafe { Tim::STimCcmr2Output::take() };
        tim_ccmr2_output.modify_reg(|r, v| r.cc4s().write(v, 0b00));
    }

    fn configure_input<Sel: InputSelection>(_tim: &GeneralTimDiverged<Tim>, _sel: Sel) {
        use drone_core::token::Token;
        let tim_ccmr2_output = unsafe { Tim::STimCcmr2Output::take() };
        tim_ccmr2_output.modify_reg(|r, v| r.cc4s().write(v, Sel::CC_SEL));
    }

    fn get_cce(tim_ccer: Tim::CTimCcer) -> bool {
        tim_ccer.cc1e().read_bit()
    }

    fn set_cce(tim_ccer: Tim::CTimCcer) {
        tim_ccer.cc4e().set_bit();
    }

    fn clear_cce(tim_ccer: Tim::CTimCcer) {
        tim_ccer.cc4e().clear_bit();
    }

    fn set_ccie(tim_dier: Tim::CTimDier) {
        tim_dier.modify_reg(|r, v| r.cc4ie().set(v));
    }

    fn clear_ccie(tim_dier: Tim::CTimDier) {
        tim_dier.modify_reg(|r, v| r.cc4ie().clear(v));
    }

    fn get_ccif(tim_sr: Tim::CTimSr) -> bool {
        tim_sr.cc4if().read_bit()
    }

    fn clear_ccif(tim_sr: Tim::CTimSr) {
        // rc_w0: Clear flag by writing a 0, 1 has no effect.
        let mut val = unsafe { Tim::STimSr::val_from(u32::MAX) };
        tim_sr.cc4if().clear(&mut val);
        tim_sr.store_val(val);
    }

    fn get_ccr(tim_ch_ccr: Self::CTimCcr) -> u32 {
        tim_ch_ccr.load_bits()
    }

    fn set_ccr(tim_ch_ccr: Self::CTimCcr, value: u32) {
        tim_ch_ccr.store_bits(value);
    }
}
