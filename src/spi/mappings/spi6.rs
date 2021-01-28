// use crate::{spi_setup, master_drv_init, pins::{*, traits::*}};
use drone_stm32_map::periph::spi::Spi6;
use drone_stm32_map::periph::dma::ch::{Dma2Ch6, Dma2Ch5};
use drone_stm32f4_dma_drv::DmaStCh1;
use drone_stm32f4_rcc_drv::clktree::PClk2;
// use drone_stm32f4_gpio_drv::pin_impl;
// use drone_stm32f4_gpio_drv::prelude::*;

spi_setup!(Spi6, PClk2);

master_drv_init!(Spi6, Dma2Ch6, DmaStCh1, Dma2Ch5, DmaStCh1);

// TODO
