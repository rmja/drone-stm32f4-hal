use crate::{
    drv::{
        DontCare, NoPull, OpenDrainType, OutputMode, PinAfToken, PinModeToken, PinPullToken,
        PinSpeed, PinTypeToken, PullDown, PullUp, PushPullType,
    },
    GpioPinCfg, GpioPinSpeed,
};
use drone_cortexm::reg::prelude::*;
use drone_stm32_map::periph::gpio::pin::GpioPinMap;

impl<Pin: GpioPinMap> GpioPinCfg<Pin, OutputMode, DontCare, DontCare> {
    /// Let pin type be push/pull.
    pub fn into_pp(self) -> GpioPinCfg<Pin, OutputMode, PushPullType, DontCare> {
        self.pin.gpio_otyper_ot.clear_bit();
        self.pin.gpio_pupdr_pupdr.write_bits(0b00); // No pull-up nor pull-down.
        GpioPinCfg::new(self.pin)
    }

    /// Let pin type be open-drain.
    pub fn into_od(self) -> GpioPinCfg<Pin, OutputMode, OpenDrainType, DontCare> {
        self.pin.gpio_otyper_ot.set_bit();
        GpioPinCfg::new(self.pin)
    }
}

impl<Pin: GpioPinMap> GpioPinCfg<Pin, OutputMode, PushPullType, DontCare> {
    /// No pull-up nor pull-down.
    pub fn into_nopull(self) -> GpioPinCfg<Pin, OutputMode, PushPullType, NoPull> {
        self.pin.gpio_pupdr_pupdr.write_bits(0b00);
        GpioPinCfg::new(self.pin)
    }

    /// Let pin be pulled-up.
    pub fn into_pullup(self) -> GpioPinCfg<Pin, OutputMode, PushPullType, PullUp> {
        self.pin.gpio_pupdr_pupdr.write_bits(0b01);
        GpioPinCfg::new(self.pin)
    }

    /// Let pin be pulled-down.
    pub fn into_pulldown(self) -> GpioPinCfg<Pin, OutputMode, PushPullType, PullDown> {
        self.pin.gpio_pupdr_pupdr.write_bits(0b10);
        GpioPinCfg::new(self.pin)
    }
}

impl<Pin: GpioPinMap, Type: PinTypeToken, Pull: PinPullToken>
    GpioPinCfg<Pin, OutputMode, Type, Pull>
{
    /// Set output pin high.
    pub fn set(&self) {
        // Set output pin by writing BS (bit set) to the bit set/reset register.
        self.pin.gpio_bsrr_bs.set_bit();
    }

    /// Set output pin low.
    pub fn clear(&self) {
        // Clear output pin by writing BR (bit reset) to the bit set/reset register.
        self.pin.gpio_bsrr_br.set_bit();
    }
}

impl<Pin: GpioPinMap, Type: PinTypeToken, Pull: PinPullToken> PinSpeed
    for GpioPinCfg<Pin, OutputMode, Type, Pull>
{
    fn with_speed(self, speed: GpioPinSpeed) -> GpioPinCfg<Pin, OutputMode, Type, Pull> {
        self.pin.gpio_ospeedr_ospeedr.write_bits(match speed {
            GpioPinSpeed::LowSpeed => 0,
            GpioPinSpeed::MediumSpeed => 1,
            GpioPinSpeed::HighSpeed => 2,
            GpioPinSpeed::VeryHighSpeed => 3,
        });
        self
    }
}
