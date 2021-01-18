use alloc::rc::Rc;
use core::{cell::RefCell, marker::PhantomData};
use drone_cortexm::reg::prelude::*;
use drone_stm32_map::periph::tim::general::{
    traits::*, GeneralTimMap, GeneralTimPeriph, TimCcmr1OutputCc2S, TimCcmr2Output,
    TimCcmr2OutputCc3S, TimCcmr2OutputCc4S,
};
use drone_stm32f4_gpio_drv::pin_ext;

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

pub struct TimChCfg<Tim: GeneralTimMap, Ch, Mode> {
    pub(crate) tim: Rc<GeneralTimPeriph<Tim>>,
    ch: PhantomData<Ch>,
    mode: PhantomData<Mode>,
}

impl<Tim: GeneralTimMap, Ch, Mode> TimChCfg<Tim, Ch, Mode> {
    pub fn new(tim: Rc<GeneralTimPeriph<Tim>>) -> Self {
        Self {
            tim,
            ch: PhantomData,
            mode: PhantomData,
        }
    }
}

impl<Tim: GeneralTimMap, Ch: TimChToken<Tim>> TimChCfg<Tim, Ch, DontCare> {
    pub fn into_output_compare(self) -> TimChCfg<Tim, Ch, OutputCompareMode> {
        Ch::configure_output(&self.tim);
        TimChCfg::new(self.tim)
    }

    pub fn into_input_capture<Sel: SelectionToken>(
        self,
        sel: Sel,
    ) -> TimChCfg<Tim, Ch, InputCaptureMode<Sel>> {
        Ch::configure_input(&self.tim, sel);
        TimChCfg::new(self.tim)
    }
}

pub trait IntoPinInputCaptureMode<
    Tim: GeneralTimMap,
    Ch: TimChToken<Tim>,
    Selection: SelectionToken,
    Pin: drone_stm32_map::periph::gpio::pin::GpioPinMap,
    Af: drone_stm32f4_gpio_drv::PinAfToken,
    Type: drone_stm32f4_gpio_drv::PinTypeToken,
    Pull: drone_stm32f4_gpio_drv::PinPullToken,
>
{
    fn into_input_capture_pin(
        self,
        pin: &drone_stm32f4_gpio_drv::GpioPin<
            Pin,
            drone_stm32f4_gpio_drv::AlternateMode<Af>,
            Type,
            Pull,
        >,
    ) -> TimChCfg<Tim, Ch, InputCaptureMode<Selection>>;
}

#[macro_export]
macro_rules! general_tim_channel {
    ($($tim_ch:ident<$tim:ident>, $pin:ident<$pin_af:ident> -> $sel:ident;)+) => {
        $(
            impl<Type: drone_stm32f4_gpio_drv::PinTypeToken, Pull: drone_stm32f4_gpio_drv::PinPullToken>
                crate::IntoPinInputCaptureMode<
                    $tim,
                    $tim_ch,
                    $sel,
                    $pin,
                    $pin_af,
                    Type,
                    Pull,
                > for TimChCfg<$tim, $tim_ch, crate::shared::DontCare>
            {
                fn into_input_capture_pin(
                    self,
                    _pin: &drone_stm32f4_gpio_drv::GpioPin<
                        $pin,
                        drone_stm32f4_gpio_drv::AlternateMode<$pin_af>,
                        Type,
                        Pull,
                    >,
                ) -> TimChCfg<$tim, $tim_ch, crate::InputCaptureMode<$sel>> {
                    self.into_input_capture($sel)
                }
            }
        )+
    };
}

pub trait TimChToken<Tim: GeneralTimMap> {
    fn configure_output(tim: &GeneralTimPeriph<Tim>);

    fn configure_input<Sel: SelectionToken>(tim: &GeneralTimPeriph<Tim>, sel: Sel);

    // /// Enable interrupt
    // fn set_ccie<Tim: GeneralTimMap>(tim: &GeneralTimPeriph<Tim>);

    // /// Disable interrupt
    // fn clear_ccie<Tim: GeneralTimMap>(tim: &GeneralTimPeriph<Tim>);
}

impl<Tim: GeneralTimMap> TimChToken<Tim> for TimCh1 {
    fn configure_output(tim: &GeneralTimPeriph<Tim>) {
        tim.tim_ccmr1_output
            .modify_reg(|r, v| r.cc1s().write(v, OutputCompareMode::CC_SEL));
    }

    fn configure_input<Sel: SelectionToken>(tim: &GeneralTimPeriph<Tim>, _sel: Sel) {
        tim.tim_ccmr1_output
            .modify_reg(|r, v| r.cc1s().write(v, Sel::CC_SEL));
    }
}

impl<Tim: GeneralTimMap + TimCcmr1OutputCc2S> TimChToken<Tim> for TimCh2 {
    fn configure_output(tim: &GeneralTimPeriph<Tim>) {
        tim.tim_ccmr1_output
            .modify_reg(|r, v| r.cc2s().write(v, OutputCompareMode::CC_SEL));
    }

    fn configure_input<Sel: SelectionToken>(tim: &GeneralTimPeriph<Tim>, _sel: Sel) {
        tim.tim_ccmr1_output
            .modify_reg(|r, v| r.cc2s().write(v, Sel::CC_SEL));
    }
}

impl<Tim: GeneralTimMap + TimCcmr2Output> TimChToken<Tim> for TimCh3 {
    fn configure_output(tim: &GeneralTimPeriph<Tim>) {
        tim.tim_ccmr2_output
            .modify_reg(|r, v| r.cc3s().write(v, OutputCompareMode::CC_SEL));
    }

    fn configure_input<Sel: SelectionToken>(tim: &GeneralTimPeriph<Tim>, _sel: Sel) {
        tim.tim_ccmr2_output
            .modify_reg(|r, v| r.cc3s().write(v, Sel::CC_SEL));
    }
}

impl<Tim: GeneralTimMap + TimCcmr2Output> TimChToken<Tim> for TimCh4 {
    fn configure_output(tim: &GeneralTimPeriph<Tim>) {
        tim.tim_ccmr2_output
            .modify_reg(|r, v| r.cc4s().write(v, OutputCompareMode::CC_SEL));
    }

    fn configure_input<Sel: SelectionToken>(tim: &GeneralTimPeriph<Tim>, _sel: Sel) {
        tim.tim_ccmr2_output
            .modify_reg(|r, v| r.cc4s().write(v, Sel::CC_SEL));
    }
}

// impl TimChToken for TimCh4 {
//     fn configure_output<Tim: GeneralTimMap>(tim: &GeneralTimPeriph<Tim>) {
//         todo!()
//     }

//     fn configure_input<Tim: GeneralTimMap, Sel: SelectionToken>(tim: &GeneralTimPeriph<Tim>, _sel: Sel) {
//         todo!()
//     }
// }

// impl<Mode: ModeToken> TimChToken for TimChCfg<TimCh1, Mode> {
//     fn configure_output<Tim: GeneralTimMap + >(tim: &GeneralTimPeriph<Tim>) {
//         tim.tim_ccmr1_output.modify_reg(|r, v| r.cc1s().write(v, 0));
//     }

//     fn configure_input<Tim: GeneralTimMap + >(tim: &GeneralTimPeriph<Tim>, sel: u8) {
//         tim.tim_ccmr1_output.modify_reg(|r, v| r.cc1s().write(v, sel as u32));
//     }

//     fn set_ccie<Tim: GeneralTimMap>(tim: &GeneralTimPeriph<Tim>) {
//         tim.tim_dier.modify_reg(|r, v| r.cc1ie().set(v));
//     }

//     fn clear_ccie<Tim: GeneralTimMap>(tim: &GeneralTimPeriph<Tim>) {
//         tim.tim_dier.modify_reg(|r, v| r.cc1ie().clear(v));
//     }
// }

// Set capture/compare register
// fn store_ccr(tim: &GeneralTimPeriph<Tim>, ccr: u16) {
//     tim.tim_ccr1.store_bits(ccr as u32);
// }

// impl<Tim: GeneralTimMap + TimCcr2 + TimDierCc2Ie> TimChCfg<Tim> for TimCh2 {
//     fn set_ccr(tim: &GeneralTimPeriph<Tim>, ccr: u16) {
//         tim.tim_ccr2.store_bits(ccr as u32);
//     }

//     fn enable_interrupt(tim: &GeneralTimPeriph<Tim>) {
//         tim.tim_dier.modify_reg(|r, v| r.cc2ie().set(v));
//     }

//     fn disable_interrupt(tim: &GeneralTimPeriph<Tim>) {
//         tim.tim_dier.modify_reg(|r, v| r.cc2ie().clear(v));
//     }
// }

// impl<Tim: GeneralTimMap + TimCcr3 + TimDierCc3Ie> TimChCfg<Tim> for TimCh3 {
//     fn set_ccr(tim: &GeneralTimPeriph<Tim>, ccr: u16) {
//         tim.tim_ccr3.store_bits(ccr as u32);
//     }

//     fn enable_interrupt(tim: &GeneralTimPeriph<Tim>) {
//         tim.tim_dier.modify_reg(|r, v| r.cc3ie().set(v));
//     }

//     fn disable_interrupt(tim: &GeneralTimPeriph<Tim>) {
//         tim.tim_dier.modify_reg(|r, v| r.cc3ie().clear(v));
//     }
// }

// impl<Tim: GeneralTimMap + TimCcr4 + TimDierCc4Ie> TimChCfg<Tim> for TimCh4 {
//     fn set_ccr(tim: &GeneralTimPeriph<Tim>, ccr: u16) {
//         tim.tim_ccr4.store_bits(ccr as u32);
//     }

//     fn enable_interrupt(tim: &GeneralTimPeriph<Tim>) {
//         tim.tim_dier.modify_reg(|r, v| r.cc4ie().set(v));
//     }

//     fn disable_interrupt(tim: &GeneralTimPeriph<Tim>) {
//         tim.tim_dier.modify_reg(|r, v| r.cc4ie().clear(v));
//     }
// }
