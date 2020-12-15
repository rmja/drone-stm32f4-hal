use crate::pins::{Defined, RxPinExt, TxPinExt, UartPins};
use drone_stm32f4_gpio_drv::pin_impl;
use drone_stm32f4_gpio_drv::prelude::*;

pin_impl!(RxPinExt for UartPins<drone_stm32_map::periph::uart::Usart2, ...>.rx, GpioA3, AlternateMode<PinAf7>; Undefined, Tx => Defined, Tx);
pin_impl!(RxPinExt for UartPins<drone_stm32_map::periph::uart::Usart2, ...>.rx, GpioD6, AlternateMode<PinAf7>; Undefined, Tx => Defined, Tx);
pin_impl!(TxPinExt for UartPins<drone_stm32_map::periph::uart::Usart2, ...>.tx, GpioA2, AlternateMode<PinAf7>; Tx, Undefined => Tx, Defined);
pin_impl!(TxPinExt for UartPins<drone_stm32_map::periph::uart::Usart2, ...>.tx, GpioD5, AlternateMode<PinAf7>; Tx, Undefined => Tx, Defined);