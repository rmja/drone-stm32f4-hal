//! STM32F4 GPIO driver for Drone OS.

use core::marker::PhantomData;
use drone_cortexm::reg::prelude::*;
use drone_stm32_map::periph::gpio::pin::{GpioPinMap, GpioPinPeriph};

/// Gpio pin configuration.
pub struct GpioPinCfg<Pin: GpioPinMap, Mode> {
    pin: GpioPinPeriph<Pin>,
    _mode: PhantomData<Mode>,
}

/// General purpose output mode.
pub struct Output<Type> {
    _type: PhantomData<Type>,
}

/// Output open-drain type.
pub struct OpenDrain;

/// Push/pull type.
pub struct PushPull;

/// Alternate function mode.
pub struct Alternate<Af, Type> {
    _fun: PhantomData<Af>,
    _type: PhantomData<Type>,
}

// pub struct AF0;
// pub struct AF1;
// pub struct AF2;
// pub struct AF3;
// pub struct AF4;
pub struct AF5;
// pub struct AF6;
pub struct AF7;
// pub struct AF8;
// pub struct AF9;
// pub struct AF10;
// pub struct AF11;
// pub struct AF12;
// pub struct AF13;
// pub struct AF14;
// pub struct AF15;

/// Generic dont-care mode for undefined state.
pub struct DontCare;

/// Gpio pin speed.
pub enum GpioPinSpeed {
    LowSpeed,
    MediumSpeed,
    HighSpeed,
    VeryHighSpeed,
}

impl<Pin: GpioPinMap> GpioPinCfg<Pin, DontCare> {
    /// Create a new pin configuration from a pin peripheral.
    pub fn from(pin: GpioPinPeriph<Pin>) -> GpioPinCfg<Pin, DontCare> {
        GpioPinCfg {
            pin,
            _mode: PhantomData,
        }
    }
}

impl<Pin: GpioPinMap> GpioPinCfg<Pin, DontCare> {
    // pub fn into_disabled(self) -> GpioPinCfg<Disabled, DontCare, DontCare> {
    //     self.periph.modify(|_r, w| w.enable.disabled());
    //     GpioPinCfg {
    //         periph: self.periph,
    //         enabled: Disabled,
    //         _direction: DontCare,
    //         mode: DontCare,
    //     }
    // }

    // pub fn into_enabled_input(self) -> GpioPinCfg<Enabled, Input, HighZ> {
    //     self.periph.modify(|_r, w| {
    //         w.enable.enabled()
    //         ._direction.input()
    //         .input_mode.high_z()
    //     });
    //     GpioPinCfg {
    //         periph: self.periph,
    //         enabled: Enabled,
    //         _direction: Input,
    //         mode: HighZ,
    //     }
    // }

    /// Set pin into general purpose output mode.
    pub fn into_output(self) -> GpioPinCfg<Pin, Output<DontCare>> {
        self.pin.gpio_moder_moder.write_bits(0b01);
        GpioPinCfg {
            pin: self.pin,
            _mode: PhantomData,
        }
    }

    // Set pin into alternate function mode, function 5.
    pub fn into_af5(self) -> GpioPinCfg<Pin, Alternate<AF5, DontCare>> {
        self.pin.gpio_afr_afr.write_bits(5);
        self.pin.gpio_moder_moder.write_bits(0b10);
        GpioPinCfg {
            pin: self.pin,
            _mode: PhantomData,
        }
    }

    // Set pin into alternate function mode, function 7.
    pub fn into_af7(self) -> GpioPinCfg<Pin, Alternate<AF7, DontCare>> {
        self.pin.gpio_afr_afr.write_bits(7);
        self.pin.gpio_moder_moder.write_bits(0b10);
        GpioPinCfg {
            pin: self.pin,
            _mode: PhantomData,
        }
    }
}

impl<Pin: GpioPinMap, Mode> GpioPinCfg<Pin, Mode> {
    pub fn pin(self) -> GpioPinPeriph<Pin> {
        self.pin
    }
}

impl<Pin: GpioPinMap> GpioPinCfg<Pin, Output<DontCare>> {
    /// Let pin output type be push/pull.
    pub fn into_pp(self) -> GpioPinCfg<Pin, Output<PushPull>> {
        self.pin.gpio_otyper_ot.clear_bit();
        self.pin.gpio_pupdr_pupdr.write_bits(0b00); // No pull-up nor pull-down.
        GpioPinCfg {
            pin: self.pin,
            _mode: PhantomData,
        }
    }

    /// Let pin output type be open-drain.
    pub fn into_od(self) -> GpioPinCfg<Pin, Output<OpenDrain>> {
        self.pin.gpio_otyper_ot.set_bit();
        GpioPinCfg {
            pin: self.pin,
            _mode: PhantomData,
        }
    }
}

impl<Pin: GpioPinMap> GpioPinCfg<Pin, Output<PushPull>> {
    /// Let pin be pulled-up.
    pub fn pull_up(self) -> GpioPinCfg<Pin, Output<PushPull>> {
        self.pin.gpio_pupdr_pupdr.write_bits(0b01);
        GpioPinCfg {
            pin: self.pin,
            _mode: PhantomData,
        }
    }

    /// Let pin be pulled-down.
    pub fn pull_down(self) -> GpioPinCfg<Pin, Output<PushPull>> {
        self.pin.gpio_pupdr_pupdr.write_bits(0b10);
        GpioPinCfg {
            pin: self.pin,
            _mode: PhantomData,
        }
    }
}

impl<Pin: GpioPinMap, Mode> GpioPinCfg<Pin, Output<Mode>> {
    /// Set pin output speed.
    pub fn with_speed(self, speed: GpioPinSpeed) -> GpioPinCfg<Pin, Output<Mode>> {
        self.pin.gpio_ospeedr_ospeedr.write_bits(match speed {
            GpioPinSpeed::LowSpeed => 0,
            GpioPinSpeed::MediumSpeed => 1,
            GpioPinSpeed::HighSpeed => 2,
            GpioPinSpeed::VeryHighSpeed => 3,
        });
        self
    }

    /// Set output pin high.
    pub fn set(&mut self) {
        // Set output pin by writing BS (bit set) to the bit set/reset register.
        self.pin.gpio_bsrr_bs.set_bit();
    }

    /// Set output pin low.
    pub fn clear(&mut self) {
        // Clear output pin by writing BR (bit reset) to the bit set/reset register.
        self.pin.gpio_bsrr_br.set_bit();
    }
}

impl<Pin: GpioPinMap, Af, Mode> GpioPinCfg<Pin, Alternate<Af, Mode>> {
    /// Let pin type be push/pull.
    pub fn into_pp(self) -> GpioPinCfg<Pin, Alternate<Af, PushPull>> {
        self.pin.gpio_otyper_ot.clear_bit();
        self.pin.gpio_pupdr_pupdr.write_bits(0b00); // No pull-up nor pull-down.
        GpioPinCfg {
            pin: self.pin,
            _mode: PhantomData,
        }
    }

    /// Let pin type be open-drain.
    pub fn into_od(self) -> GpioPinCfg<Pin, Alternate<Af, OpenDrain>> {
        self.pin.gpio_otyper_ot.set_bit();
        GpioPinCfg {
            pin: self.pin,
            _mode: PhantomData,
        }
    }

    /// Set pin speed.
    pub fn with_speed(self, speed: GpioPinSpeed) -> GpioPinCfg<Pin, Alternate<Af, Mode>> {
        self.pin.gpio_ospeedr_ospeedr.write_bits(match speed {
            GpioPinSpeed::LowSpeed => 0,
            GpioPinSpeed::MediumSpeed => 1,
            GpioPinSpeed::HighSpeed => 2,
            GpioPinSpeed::VeryHighSpeed => 3,
        });
        self
    }
}

impl<Pin: GpioPinMap, Af> GpioPinCfg<Pin, Alternate<Af, PushPull>> {
    /// Let pin be pulled-up.
    pub fn pull_up(self) -> GpioPinCfg<Pin, Alternate<Af, PushPull>> {
        self.pin.gpio_pupdr_pupdr.write_bits(0b01);
        GpioPinCfg {
            pin: self.pin,
            _mode: PhantomData,
        }
    }

    /// Let pin be pulled-down.
    pub fn pull_down(self) -> GpioPinCfg<Pin, Alternate<Af, PushPull>> {
        self.pin.gpio_pupdr_pupdr.write_bits(0b10);
        GpioPinCfg {
            pin: self.pin,
            _mode: PhantomData,
        }
    }
}
