use crate::pins::{Defined, RxPinExt, TxPinExt, UartPins};
use drone_stm32f4_dma_drv::DmaStCh4;
use drone_stm32f4_gpio_drv::pin_impl;
use drone_stm32f4_gpio_drv::prelude::*;
use drone_stm32f4_rcc_drv::clktree::PClk1;

uart_setup_init!(Usart2, PClk1);

rx_drv_init!(Usart2, Dma1Ch5, DmaStCh4);

tx_drv_init!(Usart2, Dma1Ch6, DmaStCh4);

pin_impl!(RxPinExt for UartPins<drone_stm32_map::periph::uart::Usart2, ...>.rx, GpioA3, AlternateMode<PinAf7>; Undefined, Tx => Defined, Tx);
pin_impl!(RxPinExt for UartPins<drone_stm32_map::periph::uart::Usart2, ...>.rx, GpioD6, AlternateMode<PinAf7>; Undefined, Tx => Defined, Tx);

pin_impl!(TxPinExt for UartPins<drone_stm32_map::periph::uart::Usart2, ...>.tx, GpioA2, AlternateMode<PinAf7>; Tx, Undefined => Tx, Defined);
pin_impl!(TxPinExt for UartPins<drone_stm32_map::periph::uart::Usart2, ...>.tx, GpioD5, AlternateMode<PinAf7>; Tx, Undefined => Tx, Defined);