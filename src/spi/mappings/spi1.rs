use crate::pins::{Defined, MisoPinExt, MosiPinExt, SckPinExt, SpiPins};
use drone_stm32f4_dma_drv::DmaStCh3;
use drone_stm32f4_rcc_drv::clktree::PClk2;
use drone_stm32f4_gpio_drv::pin_impl;
use drone_stm32f4_gpio_drv::prelude::*;

spi_setup!(Spi1, PClk2);

master_drv_init!(Spi1, Dma2Ch0, DmaStCh3, Dma2Ch3, DmaStCh3);
master_drv_init!(Spi1, Dma2Ch0, DmaStCh3, Dma2Ch5, DmaStCh3);
master_drv_init!(Spi1, Dma2Ch2, DmaStCh3, Dma2Ch3, DmaStCh3);
master_drv_init!(Spi1, Dma2Ch2, DmaStCh3, Dma2Ch5, DmaStCh3);

pin_impl!(SckPinExt for SpiPins<drone_stm32_map::periph::spi::Spi1, ...>.sck, GpioA5, AlternateMode<PinAf5>; Undefined, Miso, Mosi -> Defined, Miso, Mosi);
pin_impl!(SckPinExt for SpiPins<drone_stm32_map::periph::spi::Spi1, ...>.sck, GpioB3, AlternateMode<PinAf5>; Undefined, Miso, Mosi -> Defined, Miso, Mosi);

pin_impl!(MisoPinExt for SpiPins<drone_stm32_map::periph::spi::Spi1, ...>.miso, GpioA6, AlternateMode<PinAf5>; Sck, Undefined, Mosi -> Sck, Defined, Mosi);
pin_impl!(MisoPinExt for SpiPins<drone_stm32_map::periph::spi::Spi1, ...>.miso, GpioB4, AlternateMode<PinAf5>; Sck, Undefined, Mosi -> Sck, Defined, Mosi);

pin_impl!(MosiPinExt for SpiPins<drone_stm32_map::periph::spi::Spi1, ...>.mosi, GpioA7, AlternateMode<PinAf5>; Sck, Miso, Undefined -> Sck, Miso, Defined);
pin_impl!(MosiPinExt for SpiPins<drone_stm32_map::periph::spi::Spi1, ...>.mosi, GpioB5, AlternateMode<PinAf5>; Sck, Miso, Undefined -> Sck, Miso, Defined);