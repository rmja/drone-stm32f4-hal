use crate::pins::{Defined, RxPinExt, TxPinExt, UartPins};
use drone_stm32f4_gpio_drv::pin_impl;
use drone_stm32f4_gpio_drv::prelude::*;

pin_impl!(RxPinExt for UartPins<drone_stm32_map::periph::uart::Usart1, ...>.rx, GpioA10, AlternateMode<PinAf7>; Undefined, Tx => Defined, Tx);
pin_impl!(RxPinExt for UartPins<drone_stm32_map::periph::uart::Usart1, ...>.rx, GpioB7, AlternateMode<PinAf7>; Undefined, Tx => Defined, Tx);
pin_impl!(TxPinExt for UartPins<drone_stm32_map::periph::uart::Usart1, ...>.tx, GpioA9, AlternateMode<PinAf7>; Tx, Undefined => Tx, Defined);
pin_impl!(TxPinExt for UartPins<drone_stm32_map::periph::uart::Usart1, ...>.tx, GpioB6, AlternateMode<PinAf7>; Tx, Undefined => Tx, Defined);