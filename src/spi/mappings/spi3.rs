// use crate::pins::{Defined, MisoPinExt, MosiPinExt, SckPinExt, SpiPins};
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