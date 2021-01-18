use alloc::rc::Rc;
use core::{marker::PhantomData, num::NonZeroUsize};
use drone_core::fib;
use drone_cortexm::{reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::tim::general::{
    traits::*, GeneralTimMap, GeneralTimPeriph, TimCcmr1OutputCc2S, TimCcmr2Output, TimCcr2,
    TimCcr3, TimCcr4, TimDierCc2Ie, TimDierCc3Ie, TimDierCc4Ie, TimSrCc2If, TimSrCc3If, TimSrCc4If,
};
use fib::Fiber;
use futures::Stream;

use crate::shared::DontCare;

pub struct TimCh1;
pub struct TimCh2;
pub struct TimCh3;
pub struct TimCh4;

/// Timer Output Compare mode.
pub struct OutputCompareMode;

/// Timer Input Capture mode.
pub struct InputCaptureMode<Selection: SelectionToken>(PhantomData<Selection>);

pub trait ModeToken {
    const CC_SEL: u32;
}
impl ModeToken for OutputCompareMode {
    const CC_SEL: u32 = 0b00;
}
impl<Sel: SelectionToken> ModeToken for InputCaptureMode<Sel> {
    const CC_SEL: u32 = Sel::CC_SEL;
}

pub struct CaptureOverflow;

/// Channel X maps directly to the input for channel X.
pub struct DirectSelection;
/// Channel X maps to its indirect neighbour channel Y.
pub struct IndirectSelection;
/// TRC selection
pub struct TrcSelection;

pub trait SelectionToken {
    const CC_SEL: u32;
}
impl SelectionToken for DirectSelection {
    const CC_SEL: u32 = 0b01;
}
impl SelectionToken for IndirectSelection {
    const CC_SEL: u32 = 0b10;
}
impl SelectionToken for TrcSelection {
    const CC_SEL: u32 = 0b11;
}

pub struct TimChCfg<Tim: GeneralTimMap, Int: IntToken, Ch, Mode> {
    pub(crate) tim: Rc<GeneralTimPeriph<Tim>>,
    pub(crate) tim_int: Int,
    ch: PhantomData<Ch>,
    mode: PhantomData<Mode>,
}

impl<Tim: GeneralTimMap, Int: IntToken, Ch, Mode> TimChCfg<Tim, Int, Ch, Mode> {
    pub(crate) fn new(tim: Rc<GeneralTimPeriph<Tim>>, tim_int: Int) -> Self {
        Self {
            tim,
            tim_int,
            ch: PhantomData,
            mode: PhantomData,
        }
    }
}

impl<Tim: GeneralTimMap, Int: IntToken, Ch: TimChToken<Tim>> TimChCfg<Tim, Int, Ch, DontCare> {
    /// Configure the channel as Output/Compare.
    pub fn into_output_compare(self) -> TimChCfg<Tim, Int, Ch, OutputCompareMode> {
        Ch::configure_output(&self.tim);
        TimChCfg::new(self.tim, self.tim_int)
    }

    /// Configure the channel as Input/Capture.
    pub fn into_input_capture<Sel: SelectionToken>(
        self,
        sel: Sel,
    ) -> TimChCfg<Tim, Int, Ch, InputCaptureMode<Sel>> {
        Ch::configure_input(&self.tim, sel);
        TimChCfg::new(self.tim, self.tim_int)
    }
}

pub trait IntoPinInputCaptureMode<
    Tim: GeneralTimMap,
    Int: IntToken,
    Ch: TimChToken<Tim>,
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
        pin: &drone_stm32f4_gpio_drv::GpioPin<
            Pin,
            drone_stm32f4_gpio_drv::AlternateMode<Af>,
            Type,
            Pull,
        >,
    ) -> TimChCfg<Tim, Int, Ch, InputCaptureMode<Selection>>;
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
                > for TimChCfg<$tim, Int, $tim_ch, crate::shared::DontCare>
            {
                fn into_input_capture_pin(
                    self,
                    _pin: &drone_stm32f4_gpio_drv::GpioPin<
                        $pin,
                        drone_stm32f4_gpio_drv::AlternateMode<$pin_af>,
                        Type,
                        Pull,
                    >,
                ) -> TimChCfg<$tim, Int, $tim_ch, crate::InputCaptureMode<$sel>> {
                    self.into_input_capture($sel)
                }
            }
        )+
    };
}

impl<Tim: GeneralTimMap, Int: IntToken, Ch: TimChToken<Tim>, Mode: ModeToken>
    TimChCfg<Tim, Int, Ch, Mode>
{
    /// Enable channel interrupt.
    pub fn enable_interrupt(&mut self) {
        Ch::set_ccie(&self.tim);
    }

    /// Disable channel interrupt.
    pub fn disable_interrupt(&mut self) {
        Ch::clear_ccie(&self.tim);
    }

    /// Get the channel pending interrupt flag.
    pub fn is_pending(&self) -> bool {
        Ch::get_ccif(&self.tim)
    }

    /// Clear the channel pending interrupt flag.
    pub fn clear_pending(&mut self) {
        Ch::clear_ccif(&self.tim);
    }
}

impl<Tim: GeneralTimMap, Int: IntToken, Ch: TimChToken<Tim>>
    TimChCfg<Tim, Int, Ch, OutputCompareMode>
{
    /// Set the channel compare value.
    pub fn set_compare(&mut self, value: u32) {
        Ch::set_ccr(&self.tim, value);
    }
}

impl<
        Tim: GeneralTimMap,
        Int: IntToken,
        Ch: TimChToken<Tim> + 'static,
        Sel: SelectionToken + 'static,
    > TimChCfg<Tim, Int, Ch, InputCaptureMode<Sel>>
{
    /// Get the channel capture value.
    pub fn capture(&mut self) -> u32 {
        Ch::get_ccr(&self.tim)
    }

    /// Returns a stream of pulses that are generated on each channel capture. Fails on overflow.
    pub fn capture_pulse_try_stream(
        &self,
    ) -> impl Stream<Item = Result<NonZeroUsize, CaptureOverflow>> {
        self.tim_int
            .add_pulse_try_stream(|| Err(CaptureOverflow), Self::capture_fiber())
    }

    /// Returns a stream of pulses that are generated on each channel capture. Overflows are ignored.
    pub fn capture_saturating_pulse_stream(&self) -> impl Stream<Item = NonZeroUsize> {
        self.tim_int
            .add_saturating_pulse_stream(Self::capture_fiber())
    }

    fn capture_fiber<T>() -> impl Fiber<Input = (), Yield = Option<usize>, Return = T> {
        fib::new_fn(move || {
            // if self.is_pending() {
            //     self.clear_pending();
            //     fib::Yielded(Some(1))
            // }
            // else {
            fib::Yielded(None)
            // }
        })
    }
}

pub trait TimChToken<Tim: GeneralTimMap> {
    fn configure_output(tim: &GeneralTimPeriph<Tim>);
    fn configure_input<Sel: SelectionToken>(tim: &GeneralTimPeriph<Tim>, sel: Sel);

    fn set_ccie(tim: &GeneralTimPeriph<Tim>);
    fn clear_ccie(tim: &GeneralTimPeriph<Tim>);
    fn get_ccif(tim: &GeneralTimPeriph<Tim>) -> bool;
    fn clear_ccif(tim: &GeneralTimPeriph<Tim>);

    fn get_ccr(tim: &GeneralTimPeriph<Tim>) -> u32;
    fn set_ccr(tim: &GeneralTimPeriph<Tim>, value: u32);
}

impl<Tim: GeneralTimMap> TimChToken<Tim> for TimCh1 {
    fn set_ccie(tim: &GeneralTimPeriph<Tim>) {
        tim.tim_dier.modify_reg(|r, v| r.cc1ie().set(v));
    }

    fn clear_ccie(tim: &GeneralTimPeriph<Tim>) {
        tim.tim_dier.modify_reg(|r, v| r.cc1ie().clear(v));
    }

    fn get_ccif(tim: &GeneralTimPeriph<Tim>) -> bool {
        tim.tim_sr.cc1if().read_bit()
    }

    fn clear_ccif(tim: &GeneralTimPeriph<Tim>) {
        // rc_w0: Clear flag by writing a 0, 1 has no effect.
        let mut val = unsafe { Tim::STimSr::val_from(u32::MAX) };
        tim.tim_sr.cc1if().clear(&mut val);
        tim.tim_sr.store_val(val);
    }

    fn configure_output(tim: &GeneralTimPeriph<Tim>) {
        tim.tim_ccmr1_output
            .modify_reg(|r, v| r.cc1s().write(v, OutputCompareMode::CC_SEL));
    }

    fn configure_input<Sel: SelectionToken>(tim: &GeneralTimPeriph<Tim>, _sel: Sel) {
        tim.tim_ccmr1_output
            .modify_reg(|r, v| r.cc1s().write(v, Sel::CC_SEL));
    }

    fn get_ccr(tim: &GeneralTimPeriph<Tim>) -> u32 {
        tim.tim_ccr1.load_bits()
    }

    fn set_ccr(tim: &GeneralTimPeriph<Tim>, value: u32) {
        tim.tim_ccr1.store_bits(value);
    }
}

impl<Tim: GeneralTimMap + TimCcmr1OutputCc2S + TimDierCc2Ie + TimSrCc2If + TimCcr2> TimChToken<Tim>
    for TimCh2
{
    fn configure_output(tim: &GeneralTimPeriph<Tim>) {
        tim.tim_ccmr1_output
            .modify_reg(|r, v| r.cc2s().write(v, OutputCompareMode::CC_SEL));
    }

    fn configure_input<Sel: SelectionToken>(tim: &GeneralTimPeriph<Tim>, _sel: Sel) {
        tim.tim_ccmr1_output
            .modify_reg(|r, v| r.cc2s().write(v, Sel::CC_SEL));
    }

    fn set_ccie(tim: &GeneralTimPeriph<Tim>) {
        tim.tim_dier.modify_reg(|r, v| r.cc2ie().set(v));
    }

    fn clear_ccie(tim: &GeneralTimPeriph<Tim>) {
        tim.tim_dier.modify_reg(|r, v| r.cc2ie().clear(v));
    }

    fn get_ccif(tim: &GeneralTimPeriph<Tim>) -> bool {
        tim.tim_sr.cc2if().read_bit()
    }

    fn clear_ccif(tim: &GeneralTimPeriph<Tim>) {
        // rc_w0: Clear flag by writing a 0, 1 has no effect.
        let mut val = unsafe { Tim::STimSr::val_from(u32::MAX) };
        tim.tim_sr.cc2if().clear(&mut val);
        tim.tim_sr.store_val(val);
    }

    fn get_ccr(tim: &GeneralTimPeriph<Tim>) -> u32 {
        tim.tim_ccr2.load_bits()
    }

    fn set_ccr(tim: &GeneralTimPeriph<Tim>, value: u32) {
        tim.tim_ccr2.store_bits(value);
    }
}

impl<Tim: GeneralTimMap + TimCcmr2Output + TimDierCc3Ie + TimSrCc3If + TimCcr3> TimChToken<Tim>
    for TimCh3
{
    fn configure_output(tim: &GeneralTimPeriph<Tim>) {
        tim.tim_ccmr2_output
            .modify_reg(|r, v| r.cc3s().write(v, OutputCompareMode::CC_SEL));
    }

    fn configure_input<Sel: SelectionToken>(tim: &GeneralTimPeriph<Tim>, _sel: Sel) {
        tim.tim_ccmr2_output
            .modify_reg(|r, v| r.cc3s().write(v, Sel::CC_SEL));
    }

    fn set_ccie(tim: &GeneralTimPeriph<Tim>) {
        tim.tim_dier.modify_reg(|r, v| r.cc3ie().set(v));
    }

    fn clear_ccie(tim: &GeneralTimPeriph<Tim>) {
        tim.tim_dier.modify_reg(|r, v| r.cc3ie().clear(v));
    }

    fn get_ccif(tim: &GeneralTimPeriph<Tim>) -> bool {
        tim.tim_sr.cc3if().read_bit()
    }

    fn clear_ccif(tim: &GeneralTimPeriph<Tim>) {
        // rc_w0: Clear flag by writing a 0, 1 has no effect.
        let mut val = unsafe { Tim::STimSr::val_from(u32::MAX) };
        tim.tim_sr.cc3if().clear(&mut val);
        tim.tim_sr.store_val(val);
    }

    fn get_ccr(tim: &GeneralTimPeriph<Tim>) -> u32 {
        tim.tim_ccr3.load_bits()
    }

    fn set_ccr(tim: &GeneralTimPeriph<Tim>, value: u32) {
        tim.tim_ccr3.store_bits(value);
    }
}

impl<Tim: GeneralTimMap + TimCcmr2Output + TimDierCc4Ie + TimSrCc4If + TimCcr4> TimChToken<Tim>
    for TimCh4
{
    fn configure_output(tim: &GeneralTimPeriph<Tim>) {
        tim.tim_ccmr2_output
            .modify_reg(|r, v| r.cc4s().write(v, OutputCompareMode::CC_SEL));
    }

    fn configure_input<Sel: SelectionToken>(tim: &GeneralTimPeriph<Tim>, _sel: Sel) {
        tim.tim_ccmr2_output
            .modify_reg(|r, v| r.cc4s().write(v, Sel::CC_SEL));
    }

    fn set_ccie(tim: &GeneralTimPeriph<Tim>) {
        tim.tim_dier.modify_reg(|r, v| r.cc4ie().set(v));
    }

    fn clear_ccie(tim: &GeneralTimPeriph<Tim>) {
        tim.tim_dier.modify_reg(|r, v| r.cc4ie().clear(v));
    }

    fn get_ccif(tim: &GeneralTimPeriph<Tim>) -> bool {
        tim.tim_sr.cc4if().read_bit()
    }

    fn clear_ccif(tim: &GeneralTimPeriph<Tim>) {
        // rc_w0: Clear flag by writing a 0, 1 has no effect.
        let mut val = unsafe { Tim::STimSr::val_from(u32::MAX) };
        tim.tim_sr.cc4if().clear(&mut val);
        tim.tim_sr.store_val(val);
    }

    fn get_ccr(tim: &GeneralTimPeriph<Tim>) -> u32 {
        tim.tim_ccr4.load_bits()
    }

    fn set_ccr(tim: &GeneralTimPeriph<Tim>, value: u32) {
        tim.tim_ccr4.store_bits(value);
    }
}
