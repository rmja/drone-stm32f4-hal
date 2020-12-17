use crate::pins::{Defined, RxPinExt, TxPinExt, UartPins};
use drone_stm32_map::periph::gpio::pin::*;
use drone_stm32f4_dma_drv::{DmaStCh4, DmaStCh7};
use drone_stm32f4_gpio_drv::pin_impl;
use drone_stm32f4_gpio_drv::prelude::*;
use drone_stm32f4_rcc_drv::clktree::PClk1;

uart_setup_init!(Usart3, PClk1);

rx_drv_init!(Usart3, Dma1Ch1, DmaStCh4);

tx_drv_init!(Usart3, Dma1Ch3, DmaStCh4);
tx_drv_init!(Usart3, Dma1Ch4, DmaStCh7);

pin_impl!(RxPinExt for UartPins<drone_stm32_map::periph::uart::Usart3, ...>.rx, GpioB11, AlternateMode<PinAf7>; Undefined, Tx -> Defined, Tx);
pin_impl!(RxPinExt for UartPins<drone_stm32_map::periph::uart::Usart3, ...>.rx, GpioC11, AlternateMode<PinAf7>; Undefined, Tx -> Defined, Tx);
pin_impl!(RxPinExt for UartPins<drone_stm32_map::periph::uart::Usart3, ...>.rx, GpioD9, AlternateMode<PinAf7>; Undefined, Tx -> Defined, Tx);

pin_impl!(TxPinExt for UartPins<drone_stm32_map::periph::uart::Usart3, ...>.tx, GpioB10, AlternateMode<PinAf7>; Tx, Undefined -> Tx, Defined);
pin_impl!(TxPinExt for UartPins<drone_stm32_map::periph::uart::Usart3, ...>.tx, GpioC10, AlternateMode<PinAf7>; Tx, Undefined -> Tx, Defined);
pin_impl!(TxPinExt for UartPins<drone_stm32_map::periph::uart::Usart3, ...>.tx, GpioD8, AlternateMode<PinAf7>; Tx, Undefined -> Tx, Defined);
