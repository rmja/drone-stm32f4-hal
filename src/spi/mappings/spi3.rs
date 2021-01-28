use crate::{spi_setup, master_drv_init};
use drone_stm32_map::periph::spi::Spi3;
use drone_stm32_map::periph::dma::ch::{Dma1Ch0, Dma1Ch2, Dma1Ch5, Dma1Ch7};
use drone_stm32f4_dma_drv::DmaStCh0;
use drone_stm32f4_rcc_drv::clktree::PClk1;
// use drone_stm32f4_gpio_drv::pin_impl;
// use drone_stm32f4_gpio_drv::prelude::*;

spi_setup!(Spi3, PClk1);

master_drv_init!(Spi3, Dma1Ch0, DmaStCh0, Dma1Ch5, DmaStCh0);
master_drv_init!(Spi3, Dma1Ch0, DmaStCh0, Dma1Ch7, DmaStCh0);
master_drv_init!(Spi3, Dma1Ch2, DmaStCh0, Dma1Ch5, DmaStCh0);
master_drv_init!(Spi3, Dma1Ch2, DmaStCh0, Dma1Ch7, DmaStCh0);

// TODO
