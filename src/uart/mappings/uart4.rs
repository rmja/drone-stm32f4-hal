use crate::pins::{Defined, RxPinExt, TxPinExt, UartPins};
use drone_stm32f4_gpio_drv::pin_impl;
use drone_stm32f4_gpio_drv::prelude::*;

pin_impl!(RxPinExt for UartPins<drone_stm32_map::periph::uart::Uart4, ...>.rx, GpioA1, AlternateMode<PinAf8>; Undefined, Tx => Defined, Tx);
pin_impl!(RxPinExt for UartPins<drone_stm32_map::periph::uart::Uart4, ...>.rx, GpioC11, AlternateMode<PinAf8>; Undefined, Tx => Defined, Tx);
pin_impl!(TxPinExt for UartPins<drone_stm32_map::periph::uart::Uart4, ...>.tx, GpioA0, AlternateMode<PinAf8>; Tx, Undefined => Tx, Defined);
pin_impl!(TxPinExt for UartPins<drone_stm32_map::periph::uart::Uart4, ...>.tx, GpioC10, AlternateMode<PinAf8>; Tx, Undefined => Tx, Defined);