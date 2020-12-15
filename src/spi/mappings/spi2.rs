use crate::pins::{Defined, MisoPinExt, MosiPinExt, SckPinExt, SpiPins};
use drone_stm32f4_gpio_drv::pin_impl;
use drone_stm32f4_gpio_drv::prelude::*;

pin_impl!(SckPinExt for SpiPins<drone_stm32_map::periph::spi::Spi2, ...>.sck, GpioB10, AlternateMode<PinAf5>; Undefined, Miso, Mosi => Defined, Miso, Mosi);
pin_impl!(SckPinExt for SpiPins<drone_stm32_map::periph::spi::Spi2, ...>.sck, GpioB13, AlternateMode<PinAf5>; Undefined, Miso, Mosi => Defined, Miso, Mosi);
pin_impl!(SckPinExt for SpiPins<drone_stm32_map::periph::spi::Spi2, ...>.sck, GpioD3, AlternateMode<PinAf5>; Undefined, Miso, Mosi => Defined, Miso, Mosi);
pin_impl!(SckPinExt for SpiPins<drone_stm32_map::periph::spi::Spi2, ...>.sck, GpioI1, AlternateMode<PinAf5>; Undefined, Miso, Mosi => Defined, Miso, Mosi);

pin_impl!(MisoPinExt for SpiPins<drone_stm32_map::periph::spi::Spi2, ...>.miso, GpioB14, AlternateMode<PinAf5>; Sck, Undefined, Mosi => Sck, Defined, Mosi);
pin_impl!(MisoPinExt for SpiPins<drone_stm32_map::periph::spi::Spi2, ...>.miso, GpioC2, AlternateMode<PinAf5>; Sck, Undefined, Mosi => Sck, Defined, Mosi);
pin_impl!(MisoPinExt for SpiPins<drone_stm32_map::periph::spi::Spi2, ...>.miso, GpioI2, AlternateMode<PinAf5>; Sck, Undefined, Mosi => Sck, Defined, Mosi);

pin_impl!(MosiPinExt for SpiPins<drone_stm32_map::periph::spi::Spi2, ...>.mosi, GpioB15, AlternateMode<PinAf5>; Sck, Miso, Undefined => Sck, Miso, Defined);
pin_impl!(MosiPinExt for SpiPins<drone_stm32_map::periph::spi::Spi2, ...>.mosi, GpioC3, AlternateMode<PinAf5>; Sck, Miso, Undefined => Sck, Miso, Defined);
pin_impl!(MosiPinExt for SpiPins<drone_stm32_map::periph::spi::Spi2, ...>.mosi, GpioI3, AlternateMode<PinAf5>; Sck, Miso, Undefined => Sck, Miso, Defined);