use crate::{
    pin::{
        DontCare, NoPull, OpenDrainType, OutputMode, PinPullToken, PinSpeed, PinTypeToken,
        PullDown, PullUp, PushPullType,
    },
    GpioPin, GpioPinSpeed,
};
use drone_cortexm::reg::prelude::*;
use drone_stm32_map::periph::gpio::pin::GpioPinMap;

impl<Pin: GpioPinMap> GpioPin<Pin, OutputMode, DontCare, DontCare> {
    /// Let pin type be push/pull.
    pub fn into_pp(self) -> GpioPin<Pin, OutputMode, PushPullType, DontCare> {
        self.pin.gpio_otyper_ot.clear_bit();
        self.pin.gpio_pupdr_pupdr.write_bits(0b00); // No pull-up nor pull-down.
        self.pin.into()
    }

    /// Let pin type be open-drain.
    pub fn into_od(self) -> GpioPin<Pin, OutputMode, OpenDrainType, DontCare> {
        self.pin.gpio_otyper_ot.set_bit();
        self.pin.into()
    }
}

impl<Pin: GpioPinMap> GpioPin<Pin, OutputMode, PushPullType, DontCare> {
    /// No pull-up nor pull-down.
    pub fn into_nopull(self) -> GpioPin<Pin, OutputMode, PushPullType, NoPull> {
        self.pin.gpio_pupdr_pupdr.write_bits(0b00);
        self.pin.into()
    }

    /// Let pin be pulled-up.
    pub fn into_pullup(self) -> GpioPin<Pin, OutputMode, PushPullType, PullUp> {
        self.pin.gpio_pupdr_pupdr.write_bits(0b01);
        self.pin.into()
    }

    /// Let pin be pulled-down.
    pub fn into_pulldown(self) -> GpioPin<Pin, OutputMode, PushPullType, PullDown> {
        self.pin.gpio_pupdr_pupdr.write_bits(0b10);
        self.pin.into()
    }
}

impl<Pin: GpioPinMap, Type: PinTypeToken, Pull: PinPullToken> GpioPin<Pin, OutputMode, Type, Pull> {
    /// Set output pin high.
    pub fn set(&self) {
        // Set output pin to high by writing BS (bit set) to the bit set/reset register.
        self.pin.gpio_bsrr_bs.set_bit();
    }

    /// Set output pin low.
    pub fn clear(&self) {
        // Clear output pin to low by writing BR (bit reset) to the bit set/reset register.
        self.pin.gpio_bsrr_br.set_bit();
    }
}

impl<Pin: GpioPinMap, Type: PinTypeToken, Pull: PinPullToken> PinSpeed
    for GpioPin<Pin, OutputMode, Type, Pull>
{
    fn with_speed(self, speed: GpioPinSpeed) -> GpioPin<Pin, OutputMode, Type, Pull> {
        self.pin.gpio_ospeedr_ospeedr.write_bits(match speed {
            GpioPinSpeed::LowSpeed => 0,
            GpioPinSpeed::MediumSpeed => 1,
            GpioPinSpeed::HighSpeed => 2,
            GpioPinSpeed::VeryHighSpeed => 3,
        });
        self
    }
}
