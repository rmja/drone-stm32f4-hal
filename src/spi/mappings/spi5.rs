// use crate::pins::{Defined, MisoPinExt, MosiPinExt, SckPinExt, SpiPins};
use drone_stm32f4_dma_drv::{DmaStCh2, DmaStCh7};
use drone_stm32f4_rcc_drv::clktree::PClk2;
// use drone_stm32f4_gpio_drv::pin_impl;
// use drone_stm32f4_gpio_drv::prelude::*;

spi_setup!(Spi5, PClk2);

master_drv_init!(Spi5, Dma2Ch3, DmaStCh2, Dma2Ch4, DmaStCh2);
master_drv_init!(Spi5, Dma2Ch3, DmaStCh2, Dma2Ch6, DmaStCh7);
master_drv_init!(Spi5, Dma2Ch5, DmaStCh7, Dma2Ch4, DmaStCh2);
master_drv_init!(Spi5, Dma2Ch5, DmaStCh7, Dma2Ch6, DmaStCh7);

// TODO
