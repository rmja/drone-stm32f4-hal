//! STM32F4 GPIO driver for Drone OS.

use crate::head::GpioHead;
use core::marker::PhantomData;
use drone_cortexm::reg::prelude::*;
use drone_stm32_map::periph::gpio::{
    pin::{GpioPinMap, GpioPinPeriph},
    head::GpioHeadMap,
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

macro_rules! pin_init {
    ($head:ident, $pin:ident) => {
        impl NewPin<drone_stm32_map::periph::gpio::head::$head, drone_stm32_map::periph::gpio::pin::$pin>
            for GpioPin<drone_stm32_map::periph::gpio::pin::$pin, DontCare, DontCare, DontCare>
        {
            fn new(
                _head: &GpioHead<drone_stm32_map::periph::gpio::head::$head>,
                pin: GpioPinPeriph<drone_stm32_map::periph::gpio::pin::$pin>,
            ) -> Self
            {
                GpioPin::from(pin)
            }
        }
    };
}

pin_init!(GpioAHead, GpioA0);
pin_init!(GpioAHead, GpioA1);
pin_init!(GpioAHead, GpioA2);
pin_init!(GpioAHead, GpioA3);
pin_init!(GpioAHead, GpioA4);
pin_init!(GpioAHead, GpioA5);
pin_init!(GpioAHead, GpioA6);
pin_init!(GpioAHead, GpioA7);
pin_init!(GpioAHead, GpioA8);
pin_init!(GpioAHead, GpioA9);
pin_init!(GpioAHead, GpioA10);
pin_init!(GpioAHead, GpioA11);
pin_init!(GpioAHead, GpioA12);
pin_init!(GpioAHead, GpioA13);
pin_init!(GpioAHead, GpioA14);
pin_init!(GpioAHead, GpioA15);
pin_init!(GpioBHead, GpioB0);
pin_init!(GpioBHead, GpioB1);
pin_init!(GpioBHead, GpioB2);
pin_init!(GpioBHead, GpioB3);
pin_init!(GpioBHead, GpioB4);
pin_init!(GpioBHead, GpioB5);
pin_init!(GpioBHead, GpioB6);
pin_init!(GpioBHead, GpioB7);
pin_init!(GpioBHead, GpioB8);
pin_init!(GpioBHead, GpioB9);
pin_init!(GpioBHead, GpioB10);
pin_init!(GpioBHead, GpioB11);
pin_init!(GpioBHead, GpioB12);
pin_init!(GpioBHead, GpioB13);
pin_init!(GpioBHead, GpioB14);
pin_init!(GpioBHead, GpioB15);
pin_init!(GpioCHead, GpioC0);
pin_init!(GpioCHead, GpioC1);
pin_init!(GpioCHead, GpioC2);
pin_init!(GpioCHead, GpioC3);
pin_init!(GpioCHead, GpioC4);
pin_init!(GpioCHead, GpioC5);
pin_init!(GpioCHead, GpioC6);
pin_init!(GpioCHead, GpioC7);
pin_init!(GpioCHead, GpioC8);
pin_init!(GpioCHead, GpioC9);
pin_init!(GpioCHead, GpioC10);
pin_init!(GpioCHead, GpioC11);
pin_init!(GpioCHead, GpioC12);
pin_init!(GpioCHead, GpioC13);
pin_init!(GpioCHead, GpioC14);
pin_init!(GpioCHead, GpioC15);
pin_init!(GpioDHead, GpioD0);
pin_init!(GpioDHead, GpioD1);
pin_init!(GpioDHead, GpioD2);
pin_init!(GpioDHead, GpioD3);
pin_init!(GpioDHead, GpioD4);
pin_init!(GpioDHead, GpioD5);
pin_init!(GpioDHead, GpioD6);
pin_init!(GpioDHead, GpioD7);
pin_init!(GpioDHead, GpioD8);
pin_init!(GpioDHead, GpioD9);
pin_init!(GpioDHead, GpioD10);
pin_init!(GpioDHead, GpioD11);
pin_init!(GpioDHead, GpioD12);
pin_init!(GpioDHead, GpioD13);
pin_init!(GpioDHead, GpioD14);
pin_init!(GpioDHead, GpioD15);
pin_init!(GpioEHead, GpioE0);
pin_init!(GpioEHead, GpioE1);
pin_init!(GpioEHead, GpioE2);
pin_init!(GpioEHead, GpioE3);
pin_init!(GpioEHead, GpioE4);
pin_init!(GpioEHead, GpioE5);
pin_init!(GpioEHead, GpioE6);
pin_init!(GpioEHead, GpioE7);
pin_init!(GpioEHead, GpioE8);
pin_init!(GpioEHead, GpioE9);
pin_init!(GpioEHead, GpioE10);
pin_init!(GpioEHead, GpioE11);
pin_init!(GpioEHead, GpioE12);
pin_init!(GpioEHead, GpioE13);
pin_init!(GpioEHead, GpioE14);
pin_init!(GpioEHead, GpioE15);
pin_init!(GpioFHead, GpioF0);
pin_init!(GpioFHead, GpioF1);
pin_init!(GpioFHead, GpioF2);
pin_init!(GpioFHead, GpioF3);
pin_init!(GpioFHead, GpioF4);
pin_init!(GpioFHead, GpioF5);
pin_init!(GpioFHead, GpioF6);
pin_init!(GpioFHead, GpioF7);
pin_init!(GpioFHead, GpioF8);
pin_init!(GpioFHead, GpioF9);
pin_init!(GpioFHead, GpioF10);
pin_init!(GpioFHead, GpioF11);
pin_init!(GpioFHead, GpioF12);
pin_init!(GpioFHead, GpioF13);
pin_init!(GpioFHead, GpioF14);
pin_init!(GpioFHead, GpioF15);
pin_init!(GpioGHead, GpioG0);
pin_init!(GpioGHead, GpioG1);
pin_init!(GpioGHead, GpioG2);
pin_init!(GpioGHead, GpioG3);
pin_init!(GpioGHead, GpioG4);
pin_init!(GpioGHead, GpioG5);
pin_init!(GpioGHead, GpioG6);
pin_init!(GpioGHead, GpioG7);
pin_init!(GpioGHead, GpioG8);
pin_init!(GpioGHead, GpioG9);
pin_init!(GpioGHead, GpioG10);
pin_init!(GpioGHead, GpioG11);
pin_init!(GpioGHead, GpioG12);
pin_init!(GpioGHead, GpioG13);
pin_init!(GpioGHead, GpioG14);
pin_init!(GpioGHead, GpioG15);
pin_init!(GpioHHead, GpioH0);
pin_init!(GpioHHead, GpioH1);
pin_init!(GpioHHead, GpioH2);
pin_init!(GpioHHead, GpioH3);
pin_init!(GpioHHead, GpioH4);
pin_init!(GpioHHead, GpioH5);
pin_init!(GpioHHead, GpioH6);
pin_init!(GpioHHead, GpioH7);
pin_init!(GpioHHead, GpioH8);
pin_init!(GpioHHead, GpioH9);
pin_init!(GpioHHead, GpioH10);
pin_init!(GpioHHead, GpioH11);
pin_init!(GpioHHead, GpioH12);
pin_init!(GpioHHead, GpioH13);
pin_init!(GpioHHead, GpioH14);
pin_init!(GpioHHead, GpioH15);
pin_init!(GpioIHead, GpioI0);
pin_init!(GpioIHead, GpioI1);
pin_init!(GpioIHead, GpioI2);
pin_init!(GpioIHead, GpioI3);
pin_init!(GpioIHead, GpioI4);
pin_init!(GpioIHead, GpioI5);
pin_init!(GpioIHead, GpioI6);
pin_init!(GpioIHead, GpioI7);
pin_init!(GpioIHead, GpioI8);
pin_init!(GpioIHead, GpioI9);
pin_init!(GpioIHead, GpioI10);
pin_init!(GpioIHead, GpioI11);
pin_init!(GpioIHead, GpioI12);
pin_init!(GpioIHead, GpioI13);
pin_init!(GpioIHead, GpioI14);
pin_init!(GpioIHead, GpioI15);
pin_init!(GpioJHead, GpioJ0);
pin_init!(GpioJHead, GpioJ1);
pin_init!(GpioJHead, GpioJ2);
pin_init!(GpioJHead, GpioJ3);
pin_init!(GpioJHead, GpioJ4);
pin_init!(GpioJHead, GpioJ5);
pin_init!(GpioJHead, GpioJ6);
pin_init!(GpioJHead, GpioJ7);
pin_init!(GpioJHead, GpioJ8);
pin_init!(GpioJHead, GpioJ9);
pin_init!(GpioJHead, GpioJ10);
pin_init!(GpioJHead, GpioJ11);
pin_init!(GpioJHead, GpioJ12);
pin_init!(GpioJHead, GpioJ13);
pin_init!(GpioJHead, GpioJ14);
pin_init!(GpioJHead, GpioJ15);