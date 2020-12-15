use crate::pins::{Defined, MisoPinExt, MosiPinExt, SckPinExt, SpiPins};
use drone_stm32f4_gpio_drv::pin_impl;
use drone_stm32f4_gpio_drv::prelude::*;

pin_impl!(SckPinExt for SpiPins<drone_stm32_map::periph::spi::Spi1, ...>.sck, GpioA5, AlternateMode<PinAf5>; Undefined, Miso, Mosi => Defined, Miso, Mosi);
pin_impl!(SckPinExt for SpiPins<drone_stm32_map::periph::spi::Spi1, ...>.sck, GpioB3, AlternateMode<PinAf5>; Undefined, Miso, Mosi => Defined, Miso, Mosi);

pin_impl!(MisoPinExt for SpiPins<drone_stm32_map::periph::spi::Spi1, ...>.miso, GpioA6, AlternateMode<PinAf5>; Sck, Undefined, Mosi => Sck, Defined, Mosi);
pin_impl!(MisoPinExt for SpiPins<drone_stm32_map::periph::spi::Spi1, ...>.miso, GpioB4, AlternateMode<PinAf5>; Sck, Undefined, Mosi => Sck, Defined, Mosi);

pin_impl!(MosiPinExt for SpiPins<drone_stm32_map::periph::spi::Spi1, ...>.mosi, GpioA7, AlternateMode<PinAf5>; Sck, Miso, Undefined => Sck, Miso, Defined);
pin_impl!(MosiPinExt for SpiPins<drone_stm32_map::periph::spi::Spi1, ...>.mosi, GpioB5, AlternateMode<PinAf5>; Sck, Miso, Undefined => Sck, Miso, Defined);