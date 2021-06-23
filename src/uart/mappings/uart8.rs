use crate::{uart_setup_init, rx_drv_init, tx_drv_init, trx_drv_init, pins::{*, traits::*}};
use drone_stm32_map::periph::gpio::pin::*;
use drone_stm32_map::periph::uart::Uart8;
use drone_stm32f4_dma_drv::DmaStCh5;
use drone_stm32f4_gpio_drv::pin_impl;
use drone_stm32f4_gpio_drv::prelude::*;
use drone_stm32f4_rcc_drv::clktree::PClk1;

uart_setup_init!(Uart8, PClk1);

rx_drv_init!(Uart8; Dma1Ch6, DmaStCh5);

tx_drv_init!(Uart8; Dma1Ch0, DmaStCh5);

trx_drv_init!(Uart8; Dma1Ch0, DmaStCh5; Dma1Ch6, DmaStCh5);

pin_impl!(RxPinExt for UartPins<Uart8, ...>.rx, GpioE0, AlternateMode<PinAf8>; Undefined, Tx -> Defined, Tx);

pin_impl!(TxPinExt for UartPins<Uart8, ...>.tx, GpioE1, AlternateMode<PinAf8>; Tx, Undefined -> Tx, Defined);
