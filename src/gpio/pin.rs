//! STM32F4 GPIO driver for Drone OS.

use crate::head::GpioHead;
use core::marker::PhantomData;
use drone_cortexm::reg::prelude::*;
use drone_stm32_map::periph::gpio::{
    head::GpioHeadMap,
    pin::{GpioPinMap, GpioPinPeriph},
};

/// Pin configuration.
pub struct GpioPin<Pin: GpioPinMap, Mode: PinModeToken, Type: PinTypeToken, Pull: PinPullToken> {
    pub(crate) pin: GpioPinPeriph<Pin>,
    mode: PhantomData<Mode>,
    type_: PhantomData<Type>,
    pull: PhantomData<Pull>,
}

impl<Pin: GpioPinMap, Mode: PinModeToken, Type: PinTypeToken, Pull: PinPullToken>
    From<GpioPinPeriph<Pin>> for GpioPin<Pin, Mode, Type, Pull>
{
    fn from(pin: GpioPinPeriph<Pin>) -> Self {
        Self {
            pin,
            mode: PhantomData,
            type_: PhantomData,
            pull: PhantomData,
        }
    }
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

pub trait PinSpeed {
    /// Set pin speed.
    fn with_speed(self, speed: GpioPinSpeed) -> Self;
}

impl<Pin: GpioPinMap> GpioPin<Pin, DontCare, DontCare, DontCare> {
    // TODO: into_input()

    /// Set pin into general purpose output mode.
    pub fn into_output(self) -> GpioPin<Pin, OutputMode, DontCare, DontCare> {
        self.pin.gpio_moder_moder.write_bits(0b01);
        self.pin.into()
    }

    pub fn into_af<Af: PinAfToken>(self) -> GpioPin<Pin, AlternateMode<Af>, DontCare, DontCare> {
        self.pin.gpio_afr_afr.write_bits(Af::num());
        self.pin.gpio_moder_moder.write_bits(0b10);
        self.pin.into()
    }
}

pub trait NewPin<Head: GpioHeadMap, Pin: GpioPinMap> {
    /// Create a new pin configuration from a pin peripheral.
    fn new(head: &GpioHead<Head>, pin: GpioPinPeriph<Pin>) -> Self;
}

#[macro_export]
macro_rules! pin_init {
    ($head:ident, $pin:ident) => {
        impl
            crate::pin::NewPin<
                drone_stm32_map::periph::gpio::head::$head,
                drone_stm32_map::periph::gpio::pin::$pin,
            >
            for crate::pin::GpioPin<
                drone_stm32_map::periph::gpio::pin::$pin,
                crate::pin::DontCare,
                crate::pin::DontCare,
                crate::pin::DontCare,
            >
        {
            fn new(
                _head: &crate::head::GpioHead<drone_stm32_map::periph::gpio::head::$head>,
                pin: drone_stm32_map::periph::gpio::pin::GpioPinPeriph<
                    drone_stm32_map::periph::gpio::pin::$pin,
                >,
            ) -> Self {
                crate::pin::GpioPin::from(pin)
            }
        }
    };
}
