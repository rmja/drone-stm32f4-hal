use crate::pins::{Defined, RxPinExt, TxPinExt, UartPins};
use drone_stm32f4_dma_drv::DmaStCh5;
use drone_stm32f4_gpio_drv::pin_impl;
use drone_stm32f4_gpio_drv::prelude::*;
use drone_stm32f4_rcc_drv::clktree::PClk1;

uart_setup_init!(Uart8, PClk1);

rx_drv_init!(Uart8, Dma1Ch6, DmaStCh5);

tx_drv_init!(Uart8, Dma1Ch0, DmaStCh5);

pin_impl!(RxPinExt for UartPins<drone_stm32_map::periph::uart::Uart8, ...>.rx, GpioE0, AlternateMode<PinAf8>; Undefined, Tx => Defined, Tx);

pin_impl!(TxPinExt for UartPins<drone_stm32_map::periph::uart::Uart8, ...>.tx, GpioE1, AlternateMode<PinAf8>; Tx, Undefined => Tx, Defined);
