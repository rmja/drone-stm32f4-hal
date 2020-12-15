use crate::pins::{Defined, RxPinExt, TxPinExt, UartPins};
use drone_stm32f4_dma_drv::DmaStCh4;
use drone_stm32f4_gpio_drv::pin_impl;
use drone_stm32f4_gpio_drv::prelude::*;
use drone_stm32f4_rcc_drv::clktree::PClk1;

uart_setup_init!(Uart5, PClk1);

rx_drv_init!(Uart5, Dma1Ch0, DmaStCh4);

tx_drv_init!(Uart5, Dma1Ch7, DmaStCh4);

pin_impl!(RxPinExt for UartPins<drone_stm32_map::periph::uart::Uart5, ...>.rx, GpioD2, AlternateMode<PinAf8>; Undefined, Tx => Defined, Tx);

pin_impl!(TxPinExt for UartPins<drone_stm32_map::periph::uart::Uart5, ...>.tx, GpioC12, AlternateMode<PinAf8>; Tx, Undefined => Tx, Defined);