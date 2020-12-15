use crate::pins::{Defined, RxPinExt, TxPinExt, UartPins};
use drone_stm32f4_gpio_drv::pin_impl;
use drone_stm32f4_gpio_drv::prelude::*;

pin_impl!(RxPinExt for UartPins<drone_stm32_map::periph::uart::Usart6, ...>.rx, GpioC7, AlternateMode<PinAf8>; Undefined, Tx => Defined, Tx);
pin_impl!(RxPinExt for UartPins<drone_stm32_map::periph::uart::Usart6, ...>.rx, GpioG9, AlternateMode<PinAf8>; Undefined, Tx => Defined, Tx);
pin_impl!(TxPinExt for UartPins<drone_stm32_map::periph::uart::Usart6, ...>.tx, GpioC6, AlternateMode<PinAf8>; Tx, Undefined => Tx, Defined);
pin_impl!(TxPinExt for UartPins<drone_stm32_map::periph::uart::Usart6, ...>.tx, GpioG14, AlternateMode<PinAf8>; Tx, Undefined => Tx, Defined);