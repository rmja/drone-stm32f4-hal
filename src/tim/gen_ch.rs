use alloc::sync::Arc;
use core::{marker::PhantomData, pin::Pin};
use drone_core::fib::{self, Fiber};
use drone_cortexm::{reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::tim::general::{
    traits::*, GeneralTimMap, GeneralTimPeriph, TimCcmr1OutputCc2S, TimCcmr2Output, TimCcr2,
    TimCcr3, TimCcr4, TimDierCc2Ie, TimDierCc3Ie, TimDierCc4Ie, TimSrCc2If, TimSrCc3If,
    TimSrCc4If, TimCcerCc2E, TimCcerCc3E, TimCcerCc4E
};
use futures::Stream;

use crate::shared::DontCare;
use crate::traits::*;

/// The timer channel driver.
pub struct GeneralTimChDrv<Tim: GeneralTimMap, Int: IntToken, Ch, Mode> {
    pub(crate) tim: Arc<GeneralTimPeriph<Tim>>,
    pub(crate) tim_int: Int,
    ch: PhantomData<Ch>,
    mode: PhantomData<Mode>,
}

impl<Tim: GeneralTimMap, Int: IntToken, Ch, Mode> GeneralTimChDrv<Tim, Int, Ch, Mode> {
    pub(crate) fn new(tim: Arc<GeneralTimPeriph<Tim>>, tim_int: Int) -> Self {
        Self {
            tim,
            tim_int,
            ch: PhantomData,
            mode: PhantomData,
        }
    }
}

impl<Tim: GeneralTimMap, Int: IntToken, Ch: TimChToken + TimChExt<Tim>>
    GeneralTimChDrv<Tim, Int, Ch, DontCare>
{
    /// Configure the channel as Output/Compare.
    pub fn into_output_compare(self) -> GeneralTimChDrv<Tim, Int, Ch, OutputCompareMode> {
        Ch::configure_output(&self.tim);
        GeneralTimChDrv::new(self.tim, self.tim_int)
    }

    /// Configure the channel as Input/Capture.
    pub fn into_input_capture<Sel: SelectionToken>(
        self,
        sel: Sel,
    ) -> GeneralTimChDrv<Tim, Int, Ch, InputCaptureMode<Sel>> {
        Ch::configure_input(&self.tim, sel);
        GeneralTimChDrv::new(self.tim, self.tim_int)
    }
}

pub trait IntoPinInputCaptureMode<
    Tim: GeneralTimMap,
    Int: IntToken,
    Ch: TimChToken,
    Selection: SelectionToken,
    Pin: drone_stm32_map::periph::gpio::pin::GpioPinMap,
    Af: drone_stm32f4_gpio_drv::PinAfToken,
    Type: drone_stm32f4_gpio_drv::PinTypeToken,
    Pull: drone_stm32f4_gpio_drv::PinPullToken,
>
{
    /// Configure the channel as Input/Capture from a specific GPIO pin.
    fn into_input_capture_pin(
        self,
        pin: drone_stm32f4_gpio_drv::GpioPin<
            Pin,
            drone_stm32f4_gpio_drv::AlternateMode<Af>,
            Type,
            Pull,
        >,
    ) -> GeneralTimChDrv<Tim, Int, Ch, InputCaptureMode<Selection>>;
}

#[macro_export]
macro_rules! general_tim_channel {
    ($($tim_ch:ident<$tim:ident>, $pin:ident<$pin_af:ident> -> $sel:ident;)+) => {
        $(
            impl<Int: drone_cortexm::thr::IntToken, Type: drone_stm32f4_gpio_drv::PinTypeToken, Pull: drone_stm32f4_gpio_drv::PinPullToken>
                crate::IntoPinInputCaptureMode<
                    $tim,
                    Int,
                    $tim_ch,
                    $sel,
                    $pin,
                    $pin_af,
                    Type,
                    Pull,
                > for GeneralTimChDrv<$tim, Int, $tim_ch, crate::shared::DontCare>
            {
                fn into_input_capture_pin(
                    self,
                    _pin: drone_stm32f4_gpio_drv::GpioPin<
                        $pin,
                        drone_stm32f4_gpio_drv::AlternateMode<$pin_af>,
                        Type,
                        Pull,
                    >,
                ) -> GeneralTimChDrv<$tim, Int, $tim_ch, crate::InputCaptureMode<$sel>> {
                    self.into_input_capture($sel)
                }
            }
        )+
    };
}

impl<
        Tim: GeneralTimMap,
        Int: IntToken,
        Ch: TimChToken + TimChExt<Tim> + Send + 'static,
        Sel: SelectionToken + Send + 'static,
    > GeneralTimChDrv<Tim, Int, Ch, InputCaptureMode<Sel>>
{
    fn capture_stream<Item>(
        &mut self,
        stream_factory: impl FnOnce(
            Int,
            Tim::CTimSr,
            Ch::CTimCcr,
        ) -> Pin<Box<dyn Stream<Item = Item> + Send>>,
    ) -> CaptureStream<'_, Self, Item> {
        assert!(self.tim_int.is_int_enabled());
        use drone_core::token::Token;
        let tim_sr = unsafe { Tim::CTimSr::take() };
        let tim_ch_ccr = unsafe { Ch::CTimCcr::take() };
        let stream = stream_factory(self.tim_int, tim_sr, tim_ch_ccr);
        Ch::set_cce(&self.tim.tim_ccer);
        Ch::set_ccie(&self.tim.tim_dier);
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
        Ch: TimChToken + TimChExt<Tim> + Send + 'static,
        Sel: SelectionToken + Send + 'static,
    > TimerCaptureCh for GeneralTimChDrv<Tim, Int, Ch, InputCaptureMode<Sel>>
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

impl<
        Tim: GeneralTimMap,
        Int: IntToken,
        Ch: TimChToken + TimChExt<Tim> + Send,
        Sel: SelectionToken + Send,
    > CaptureStop for GeneralTimChDrv<Tim, Int, Ch, InputCaptureMode<Sel>>
{
    fn stop(&mut self) {
        Ch::clear_ccie(&self.tim.tim_dier);
        Ch::clear_cce(&self.tim.tim_ccer);
    }
}

pub trait TimChExt<Tim: GeneralTimMap> {
    type CTimCcr: RReg<Crt> + Copy;

    fn configure_output(tim: &GeneralTimPeriph<Tim>);
    fn configure_input<Sel: SelectionToken>(tim: &GeneralTimPeriph<Tim>, sel: Sel);

    fn set_cce(tim_ccer: &Tim::STimCcer);
    fn clear_cce(tim_ccer: &Tim::STimCcer);
    fn set_ccie(tim_dier: &Tim::STimDier);
    fn clear_ccie(tim_dier: &Tim::STimDier);
    fn get_ccif(tim_sr: Tim::CTimSr) -> bool;
    fn clear_ccif(tim_sr: Tim::CTimSr);
    fn get_ccr(tim_cr_ccr: Self::CTimCcr) -> u32;
    fn set_ccr(tim_cr_ccr: Self::CTimCcr, value: u32);
}

impl<Tim: GeneralTimMap> TimChExt<Tim> for TimCh1 {
    type CTimCcr = Tim::CTimCcr1;

    fn configure_output(tim: &GeneralTimPeriph<Tim>) {
        tim.tim_ccmr1_output
            .modify_reg(|r, v| r.cc1s().write(v, OutputCompareMode::CC_SEL));
    }

    fn configure_input<Sel: SelectionToken>(tim: &GeneralTimPeriph<Tim>, _sel: Sel) {
        tim.tim_ccmr1_output
            .modify_reg(|r, v| r.cc1s().write(v, Sel::CC_SEL));
    }

    fn set_cce(tim_ccer: &Tim::STimCcer) {
        tim_ccer.cc1e().set_bit();
    }

    fn clear_cce(tim_ccer: &Tim::STimCcer) {
        tim_ccer.cc1e().clear_bit();
    }

    fn set_ccie(tim_dier: &Tim::STimDier) {
        tim_dier.modify_reg(|r, v| r.cc1ie().set(v));
    }

    fn clear_ccie(tim_dier: &Tim::STimDier) {
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

    fn get_ccr(tim_cr_ccr: Self::CTimCcr) -> u32 {
        tim_cr_ccr.load_bits()
    }

    fn set_ccr(tim_cr_ccr: Self::CTimCcr, value: u32) {
        tim_cr_ccr.store_bits(value);
    }
}

impl<Tim: GeneralTimMap + TimCcmr1OutputCc2S + TimDierCc2Ie + TimCcerCc2E + TimSrCc2If + TimCcr2> TimChExt<Tim>
    for TimCh2
{
    type CTimCcr = Tim::CTimCcr2;

    fn configure_output(tim: &GeneralTimPeriph<Tim>) {
        tim.tim_ccmr1_output
            .modify_reg(|r, v| r.cc2s().write(v, OutputCompareMode::CC_SEL));
    }

    fn configure_input<Sel: SelectionToken>(tim: &GeneralTimPeriph<Tim>, _sel: Sel) {
        tim.tim_ccmr1_output
            .modify_reg(|r, v| r.cc2s().write(v, Sel::CC_SEL));
    }

    fn set_cce(tim_ccer: &Tim::STimCcer) {
        tim_ccer.cc2e().set_bit();
    }

    fn clear_cce(tim_ccer: &Tim::STimCcer) {
        tim_ccer.cc2e().clear_bit();
    }

    fn set_ccie(tim_dier: &Tim::STimDier) {
        tim_dier.modify_reg(|r, v| r.cc2ie().set(v));
    }

    fn clear_ccie(tim_dier: &Tim::STimDier) {
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

    fn get_ccr(tim_cr_ccr: Self::CTimCcr) -> u32 {
        tim_cr_ccr.load_bits()
    }

    fn set_ccr(tim_cr_ccr: Self::CTimCcr, value: u32) {
        tim_cr_ccr.store_bits(value);
    }
}

impl<Tim: GeneralTimMap + TimCcmr2Output + TimDierCc3Ie + TimCcerCc3E + TimSrCc3If + TimCcr3> TimChExt<Tim>
    for TimCh3
{
    type CTimCcr = Tim::CTimCcr3;

    fn configure_output(tim: &GeneralTimPeriph<Tim>) {
        tim.tim_ccmr2_output
            .modify_reg(|r, v| r.cc3s().write(v, OutputCompareMode::CC_SEL));
    }

    fn configure_input<Sel: SelectionToken>(tim: &GeneralTimPeriph<Tim>, _sel: Sel) {
        tim.tim_ccmr2_output
            .modify_reg(|r, v| r.cc3s().write(v, Sel::CC_SEL));
    }

    fn set_cce(tim_ccer: &Tim::STimCcer) {
        tim_ccer.cc3e().set_bit();
    }

    fn clear_cce(tim_ccer: &Tim::STimCcer) {
        tim_ccer.cc3e().clear_bit();
    }

    fn set_ccie(tim_dier: &Tim::STimDier) {
        tim_dier.modify_reg(|r, v| r.cc3ie().set(v));
    }

    fn clear_ccie(tim_dier: &Tim::STimDier) {
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

    fn get_ccr(tim_cr_ccr: Self::CTimCcr) -> u32 {
        tim_cr_ccr.load_bits()
    }

    fn set_ccr(tim_cr_ccr: Self::CTimCcr, value: u32) {
        tim_cr_ccr.store_bits(value);
    }
}

impl<Tim: GeneralTimMap + TimCcmr2Output + TimDierCc4Ie + TimCcerCc4E + TimSrCc4If + TimCcr4> TimChExt<Tim>
    for TimCh4
{
    type CTimCcr = Tim::CTimCcr4;

    fn configure_output(tim: &GeneralTimPeriph<Tim>) {
        tim.tim_ccmr2_output
            .modify_reg(|r, v| r.cc4s().write(v, OutputCompareMode::CC_SEL));
    }

    fn configure_input<Sel: SelectionToken>(tim: &GeneralTimPeriph<Tim>, _sel: Sel) {
        tim.tim_ccmr2_output
            .modify_reg(|r, v| r.cc4s().write(v, Sel::CC_SEL));
    }

    fn set_cce(tim_ccer: &Tim::STimCcer) {
        tim_ccer.cc4e().set_bit();
    }

    fn clear_cce(tim_ccer: &Tim::STimCcer) {
        tim_ccer.cc4e().clear_bit();
    }

    fn set_ccie(tim_dier: &Tim::STimDier) {
        tim_dier.modify_reg(|r, v| r.cc4ie().set(v));
    }

    fn clear_ccie(tim_dier: &Tim::STimDier) {
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

    fn get_ccr(tim_cr_ccr: Self::CTimCcr) -> u32 {
        tim_cr_ccr.load_bits()
    }

    fn set_ccr(tim_cr_ccr: Self::CTimCcr, value: u32) {
        tim_cr_ccr.store_bits(value);
    }
}
