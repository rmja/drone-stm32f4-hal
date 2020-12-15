use crate::pins::{Defined, MisoPinExt, MosiPinExt, SckPinExt, SpiPins};
use drone_stm32f4_dma_drv::DmaStCh1;
use drone_stm32f4_gpio_drv::pin_impl;
use drone_stm32f4_gpio_drv::prelude::*;

master_drv_init!(Spi6, Dma2Ch6, DmaStCh1, Dma2Ch5, DmaStCh1);

// TODO