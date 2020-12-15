// use crate::pins::{Defined, MisoPinExt, MosiPinExt, SckPinExt, SpiPins};
use drone_stm32f4_dma_drv::{DmaStCh4, DmaStCh5};
// use drone_stm32f4_gpio_drv::pin_impl;
// use drone_stm32f4_gpio_drv::prelude::*;

master_drv_init!(Spi4, Dma2Ch0, DmaStCh4, Dma2Ch1, DmaStCh4);
master_drv_init!(Spi4, Dma2Ch0, DmaStCh4, Dma2Ch4, DmaStCh5);
master_drv_init!(Spi4, Dma2Ch3, DmaStCh5, Dma2Ch1, DmaStCh4);
master_drv_init!(Spi4, Dma2Ch3, DmaStCh5, Dma2Ch4, DmaStCh5);

// TODO