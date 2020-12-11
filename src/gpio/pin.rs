//! STM32F4 GPIO driver for Drone OS.

use core::marker::PhantomData;
use drone_cortexm::reg::prelude::*;
use drone_stm32_map::periph::gpio::pin::{GpioPinMap, GpioPinPeriph};
use crate::head::GpioHead;

/// Pin configuration.
pub struct GpioPin<Pin: GpioPinMap, Mode: PinModeToken, Type: PinTypeToken, Pull: PinPullToken> {
    pub(crate) pin: GpioPinPeriph<Pin>,
    mode: PhantomData<Mode>,
    type_: PhantomData<Type>,
    pull: PhantomData<Pull>,
}

/// Generic dont-care mode for undefined state.
pub struct DontCare;

/// General purpose input mode (MODER=0b00).
pub struct InputMode;

/// General purpose output mode  (MODER=0b01).
pub struct OutputMode;

/// Alternate function mode  (MODER=0b10).
pub struct AlternateMode<Af: PinAfToken> {
    af: PhantomData<Af>,
}

// TODO: Analog mode

pub trait PinModeToken {}
impl PinModeToken for InputMode {}
impl PinModeToken for OutputMode {}
impl<Af: PinAfToken> PinModeToken for AlternateMode<Af> {}
impl PinModeToken for DontCare {}

/// Push/pull type (OTYPER=0).
/// This is only applicabale for OutputMode and AlternateMode.
pub struct PushPullType;

/// Output open-drain type (OTYPER=1).
/// This is only applicabale for OutputMode and AlternateMode.
pub struct OpenDrainType;

pub trait PinTypeToken {}
impl PinTypeToken for PushPullType {}
impl PinTypeToken for OpenDrainType {}
impl PinTypeToken for DontCare {}

/// No pull-up nor pull-down. For inputs this means floating.
pub struct NoPull;

/// Pull up.
pub struct PullUp;

/// Pull down.
pub struct PullDown;

pub trait PinPullToken {}
impl PinPullToken for NoPull {}
impl PinPullToken for PullUp {}
impl PinPullToken for PullDown {}
impl PinPullToken for DontCare {}

pub struct PinAf0;
pub struct PinAf1;
pub struct PinAf2;
pub struct PinAf3;
pub struct PinAf4;
pub struct PinAf5;
pub struct PinAf6;
pub struct PinAf7;
pub struct PinAf8;
pub struct PinAf9;
pub struct PinAf10;
pub struct PinAf11;
pub struct PinAf12;
pub struct PinAf13;
pub struct PinAf14;
pub struct PinAf15;

pub trait PinAfToken {
    fn num() -> u32;
}

macro_rules! af_token {
    ($af:ident, $num:expr) => {
        impl PinAfToken for $af {
            fn num() -> u32 {
                $num
            }
        }
    };
}

af_token!(PinAf0, 0);
af_token!(PinAf1, 1);
af_token!(PinAf2, 2);
af_token!(PinAf3, 3);
af_token!(PinAf4, 4);
af_token!(PinAf5, 5);
af_token!(PinAf6, 6);
af_token!(PinAf7, 7);
af_token!(PinAf8, 8);
af_token!(PinAf9, 9);
af_token!(PinAf10, 10);
af_token!(PinAf11, 11);
af_token!(PinAf12, 12);
af_token!(PinAf13, 13);
af_token!(PinAf14, 14);
af_token!(PinAf15, 15);
af_token!(DontCare, 0);

/// Gpio pin speed.
pub enum GpioPinSpeed {
    LowSpeed,
    MediumSpeed,
    HighSpeed,
    VeryHighSpeed,
}

impl<Pin: GpioPinMap, Mode: PinModeToken, Type: PinTypeToken, Pull: PinPullToken>
    GpioPin<Pin, Mode, Type, Pull>
{
    pub(crate) fn new(pin: GpioPinPeriph<Pin>) -> GpioPin<Pin, Mode, Type, Pull> {
        Self {
            pin,
            mode: PhantomData,
            type_: PhantomData,
            pull: PhantomData,
        }
    }
}


pub trait PinInit<Pin: GpioPinMap> {
    /// Create a new pin configuration from a pin peripheral.
    fn init_pin(&self, pin: GpioPinPeriph<Pin>) -> GpioPin<Pin, DontCare, DontCare, DontCare>;
}

macro_rules! pin_init {
    ($head:ident, $pin:ident) => {
        impl PinInit<drone_stm32_map::periph::gpio::pin::$pin> for GpioHead<drone_stm32_map::periph::gpio::head::$head> {
            fn init_pin(&self, pin: GpioPinPeriph<drone_stm32_map::periph::gpio::pin::$pin>) -> GpioPin<drone_stm32_map::periph::gpio::pin::$pin, DontCare, DontCare, DontCare> {
                GpioPin::new(pin)
            }
        }
    };
}

pin_init!(GpioAHead, GpioA0);
pin_init!(GpioAHead, GpioA1);
pin_init!(GpioAHead, GpioA2);
pin_init!(GpioAHead, GpioA3);

impl<Pin: GpioPinMap> GpioPin<Pin, DontCare, DontCare, DontCare> {
    /// Create a new pin configuration from a pin peripheral.
    pub fn init(pin: GpioPinPeriph<Pin>) -> GpioPin<Pin, DontCare, DontCare, DontCare> {
        Self::new(pin)
    }

    // pub fn into_disabled(self) -> GpioPin<Disabled, DontCare, DontCare> {
    //     self.periph.modify(|_r, w| w.enable.disabled());
    //     GpioPin {
    //         periph: self.periph,
    //         enabled: Disabled,
    //         _direction: DontCare,
    //         mode: DontCare,
    //     }
    // }

    // pub fn into_enabled_input(self) -> GpioPin<Enabled, Input, HighZ> {
    //     self.periph.modify(|_r, w| {
    //         w.enable.enabled()
    //         ._direction.input()
    //         .input_mode.high_z()
    //     });
    //     GpioPin {
    //         periph: self.periph,
    //         enabled: Enabled,
    //         _direction: Input,
    //         mode: HighZ,
    //     }
    // }

    /// Set pin into general purpose output mode.
    pub fn into_output(self) -> GpioPin<Pin, OutputMode, DontCare, DontCare> {
        self.pin.gpio_moder_moder.write_bits(0b01);
        GpioPin::new(self.pin)
    }

    // Set pin into alternate function mode.
    pub fn into_af<Af: PinAfToken>(self) -> GpioPin<Pin, AlternateMode<Af>, DontCare, DontCare> {
        self.pin.gpio_afr_afr.write_bits(Af::num());
        self.pin.gpio_moder_moder.write_bits(0b10);
        GpioPin::new(self.pin)
    }
}

pub trait PinSpeed {
    /// Set pin speed.
    fn with_speed(self, speed: GpioPinSpeed) -> Self;
}
