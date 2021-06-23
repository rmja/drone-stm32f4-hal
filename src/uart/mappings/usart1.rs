use crate::{uart_setup_init, rx_drv_init, tx_drv_init, trx_drv_init, pins::{*, traits::*}};
use drone_stm32_map::periph::gpio::pin::*;
use drone_stm32_map::periph::uart::Usart1;
use drone_stm32f4_dma_drv::DmaStCh4;
use drone_stm32f4_gpio_drv::pin_impl;
use drone_stm32f4_gpio_drv::prelude::*;
use drone_stm32f4_rcc_drv::clktree::PClk2;

uart_setup_init!(Usart1, PClk2);

rx_drv_init!(Usart1; Dma2Ch2, DmaStCh4);
rx_drv_init!(Usart1; Dma2Ch5, DmaStCh4);

tx_drv_init!(Usart1; Dma2Ch7, DmaStCh4);

trx_drv_init!(Usart1; Dma2Ch7, DmaStCh4; Dma2Ch2, DmaStCh4);
trx_drv_init!(Usart1; Dma2Ch7, DmaStCh4; Dma2Ch5, DmaStCh4);

pin_impl!(RxPinExt for UartPins<Usart1, ...>.rx, GpioA10, AlternateMode<PinAf7>; Undefined, Tx -> Defined, Tx);
pin_impl!(RxPinExt for UartPins<Usart1, ...>.rx, GpioB7, AlternateMode<PinAf7>; Undefined, Tx -> Defined, Tx);

pin_impl!(TxPinExt for UartPins<Usart1, ...>.tx, GpioA9, AlternateMode<PinAf7>; Tx, Undefined -> Tx, Defined);
pin_impl!(TxPinExt for UartPins<Usart1, ...>.tx, GpioB6, AlternateMode<PinAf7>; Tx, Undefined -> Tx, Defined);
