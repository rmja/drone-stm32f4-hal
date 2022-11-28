use crate::{
    master_drv_init,
    pins::{traits::*, *},
    spi_setup,
};
use drone_stm32_map::periph::dma::ch::{Dma1Ch3, Dma1Ch4};
use drone_stm32_map::periph::gpio::pin::*;
use drone_stm32_map::periph::spi::Spi2;
use drone_stm32f4_dma_drv::DmaStCh0;
use drone_stm32f4_gpio_drv::pin_impl;
use drone_stm32f4_gpio_drv::prelude::*;
use drone_stm32f4_rcc_drv::clktree::PClk2;

spi_setup!(Spi2, PClk2);

master_drv_init!(Spi2, Dma1Ch3, DmaStCh0, Dma1Ch4, DmaStCh0);

pin_impl!(SckPinExt for SpiPins<Spi2, ...>.sck, GpioB10, AlternateMode<PinAf5>; Undefined, Miso, Mosi -> Defined, Miso, Mosi);
pin_impl!(SckPinExt for SpiPins<Spi2, ...>.sck, GpioB13, AlternateMode<PinAf5>; Undefined, Miso, Mosi -> Defined, Miso, Mosi);
pin_impl!(SckPinExt for SpiPins<Spi2, ...>.sck, GpioD3, AlternateMode<PinAf5>; Undefined, Miso, Mosi -> Defined, Miso, Mosi);
#[cfg(any(
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f469",
))]
pin_impl!(SckPinExt for SpiPins<Spi2, ...>.sck, GpioI1, AlternateMode<PinAf5>; Undefined, Miso, Mosi -> Defined, Miso, Mosi);

pin_impl!(MisoPinExt for SpiPins<Spi2, ...>.miso, GpioB14, AlternateMode<PinAf5>; Sck, Undefined, Mosi -> Sck, Defined, Mosi);
pin_impl!(MisoPinExt for SpiPins<Spi2, ...>.miso, GpioC2, AlternateMode<PinAf5>; Sck, Undefined, Mosi -> Sck, Defined, Mosi);
#[cfg(any(
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f469",
))]
pin_impl!(MisoPinExt for SpiPins<Spi2, ...>.miso, GpioI2, AlternateMode<PinAf5>; Sck, Undefined, Mosi -> Sck, Defined, Mosi);

pin_impl!(MosiPinExt for SpiPins<Spi2, ...>.mosi, GpioB15, AlternateMode<PinAf5>; Sck, Miso, Undefined -> Sck, Miso, Defined);
pin_impl!(MosiPinExt for SpiPins<Spi2, ...>.mosi, GpioC3, AlternateMode<PinAf5>; Sck, Miso, Undefined -> Sck, Miso, Defined);
#[cfg(any(
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f469",
))]
pin_impl!(MosiPinExt for SpiPins<Spi2, ...>.mosi, GpioI3, AlternateMode<PinAf5>; Sck, Miso, Undefined -> Sck, Miso, Defined);
