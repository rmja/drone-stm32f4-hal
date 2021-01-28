// use crate::{spi_setup, master_drv_init, pins::{*, traits::*}};
use drone_stm32_map::periph::spi::Spi4;
use drone_stm32_map::periph::dma::ch::{Dma2Ch0, Dma2Ch1, Dma2Ch3, Dma2Ch4};
use drone_stm32f4_dma_drv::{DmaStCh4, DmaStCh5};
use drone_stm32f4_rcc_drv::clktree::PClk2;
// use drone_stm32f4_gpio_drv::pin_impl;
// use drone_stm32f4_gpio_drv::prelude::*;

spi_setup!(Spi4, PClk2);

master_drv_init!(Spi4, Dma2Ch0, DmaStCh4, Dma2Ch1, DmaStCh4);
master_drv_init!(Spi4, Dma2Ch0, DmaStCh4, Dma2Ch4, DmaStCh5);
master_drv_init!(Spi4, Dma2Ch3, DmaStCh5, Dma2Ch1, DmaStCh4);
master_drv_init!(Spi4, Dma2Ch3, DmaStCh5, Dma2Ch4, DmaStCh5);

// TODO
