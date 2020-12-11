use crate::{
    drv::{
        AlternateMode, DontCare, NoPull, OpenDrainType, PinAfToken, PinPullToken, PinSpeed,
        PinTypeToken, PullDown, PullUp, PushPullType,
    },
    GpioPinCfg, GpioPinSpeed,
};
use drone_cortexm::reg::prelude::*;
use drone_stm32_map::periph::gpio::pin::GpioPinMap;

impl<Pin: GpioPinMap, Af: PinAfToken> GpioPinCfg<Pin, AlternateMode<Af>, DontCare, DontCare> {
    /// Let pin type be push/pull.
    pub fn into_pp(self) -> GpioPinCfg<Pin, AlternateMode<Af>, PushPullType, DontCare> {
        self.pin.gpio_otyper_ot.clear_bit();
        self.pin.gpio_pupdr_pupdr.write_bits(0b00); // No pull-up nor pull-down.
        GpioPinCfg::new(self.pin)
    }

    /// Let pin type be open-drain.
    pub fn into_od(self) -> GpioPinCfg<Pin, AlternateMode<Af>, OpenDrainType, DontCare> {
        self.pin.gpio_otyper_ot.set_bit();
        GpioPinCfg::new(self.pin)
    }
}

impl<Pin: GpioPinMap, Af: PinAfToken> GpioPinCfg<Pin, AlternateMode<Af>, PushPullType, DontCare> {
    /// No pull-up nor pull-down.
    pub fn into_nopull(self) -> GpioPinCfg<Pin, AlternateMode<Af>, PushPullType, NoPull> {
        self.pin.gpio_pupdr_pupdr.write_bits(0b00);
        GpioPinCfg::new(self.pin)
    }

    /// Let pin be pulled-up.
    pub fn into_pullup(self) -> GpioPinCfg<Pin, AlternateMode<Af>, PushPullType, PullUp> {
        self.pin.gpio_pupdr_pupdr.write_bits(0b01);
        GpioPinCfg::new(self.pin)
    }

    /// Let pin be pulled-down.
    pub fn into_pulldown(self) -> GpioPinCfg<Pin, AlternateMode<Af>, PushPullType, PullDown> {
        self.pin.gpio_pupdr_pupdr.write_bits(0b10);
        GpioPinCfg::new(self.pin)
    }
}

impl<Pin: GpioPinMap, Af: PinAfToken, Type: PinTypeToken, Pull: PinPullToken> PinSpeed
    for GpioPinCfg<Pin, AlternateMode<Af>, Type, Pull>
{
    fn with_speed(self, speed: GpioPinSpeed) -> GpioPinCfg<Pin, AlternateMode<Af>, Type, Pull> {
        self.pin.gpio_ospeedr_ospeedr.write_bits(match speed {
            GpioPinSpeed::LowSpeed => 0,
            GpioPinSpeed::MediumSpeed => 1,
            GpioPinSpeed::HighSpeed => 2,
            GpioPinSpeed::VeryHighSpeed => 3,
        });
        self
    }
}
