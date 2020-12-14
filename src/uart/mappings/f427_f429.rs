use crate::pins::{Defined, RxPinExt, TxPinExt, UartPins};
use drone_stm32f4_gpio_drv::pin_impl;
use drone_stm32f4_gpio_drv::prelude::*;

pin_impl!(RxPinExt for UartPins<drone_stm32_map::periph::uart::Usart1, ...>.rx, GpioA10, AlternateMode<PinAf7>; Undefined, Tx => Defined, Tx);
pin_impl!(RxPinExt for UartPins<drone_stm32_map::periph::uart::Usart1, ...>.rx, GpioB7, AlternateMode<PinAf7>; Undefined, Tx => Defined, Tx);
pin_impl!(TxPinExt for UartPins<drone_stm32_map::periph::uart::Usart1, ...>.tx, GpioA9, AlternateMode<PinAf7>; Tx, Undefined => Tx, Defined);
pin_impl!(TxPinExt for UartPins<drone_stm32_map::periph::uart::Usart1, ...>.tx, GpioB6, AlternateMode<PinAf7>; Tx, Undefined => Tx, Defined);

pin_impl!(RxPinExt for UartPins<drone_stm32_map::periph::uart::Usart2, ...>.rx, GpioA3, AlternateMode<PinAf7>; Undefined, Tx => Defined, Tx);
pin_impl!(RxPinExt for UartPins<drone_stm32_map::periph::uart::Usart2, ...>.rx, GpioD6, AlternateMode<PinAf7>; Undefined, Tx => Defined, Tx);
pin_impl!(TxPinExt for UartPins<drone_stm32_map::periph::uart::Usart2, ...>.tx, GpioA2, AlternateMode<PinAf7>; Tx, Undefined => Tx, Defined);
pin_impl!(TxPinExt for UartPins<drone_stm32_map::periph::uart::Usart2, ...>.tx, GpioD5, AlternateMode<PinAf7>; Tx, Undefined => Tx, Defined);

pin_impl!(RxPinExt for UartPins<drone_stm32_map::periph::uart::Usart3, ...>.rx, GpioB11, AlternateMode<PinAf7>; Undefined, Tx => Defined, Tx);
pin_impl!(RxPinExt for UartPins<drone_stm32_map::periph::uart::Usart3, ...>.rx, GpioC11, AlternateMode<PinAf7>; Undefined, Tx => Defined, Tx);
pin_impl!(RxPinExt for UartPins<drone_stm32_map::periph::uart::Usart3, ...>.rx, GpioD9, AlternateMode<PinAf7>; Undefined, Tx => Defined, Tx);
pin_impl!(TxPinExt for UartPins<drone_stm32_map::periph::uart::Usart3, ...>.tx, GpioB10, AlternateMode<PinAf7>; Tx, Undefined => Tx, Defined);
pin_impl!(TxPinExt for UartPins<drone_stm32_map::periph::uart::Usart3, ...>.tx, GpioC10, AlternateMode<PinAf7>; Tx, Undefined => Tx, Defined);
pin_impl!(TxPinExt for UartPins<drone_stm32_map::periph::uart::Usart3, ...>.tx, GpioD8, AlternateMode<PinAf7>; Tx, Undefined => Tx, Defined);

pin_impl!(RxPinExt for UartPins<drone_stm32_map::periph::uart::Uart4, ...>.rx, GpioA1, AlternateMode<PinAf8>; Undefined, Tx => Defined, Tx);
pin_impl!(RxPinExt for UartPins<drone_stm32_map::periph::uart::Uart4, ...>.rx, GpioC11, AlternateMode<PinAf8>; Undefined, Tx => Defined, Tx);
pin_impl!(TxPinExt for UartPins<drone_stm32_map::periph::uart::Uart4, ...>.tx, GpioA0, AlternateMode<PinAf8>; Tx, Undefined => Tx, Defined);
pin_impl!(TxPinExt for UartPins<drone_stm32_map::periph::uart::Uart4, ...>.tx, GpioC10, AlternateMode<PinAf8>; Tx, Undefined => Tx, Defined);

pin_impl!(RxPinExt for UartPins<drone_stm32_map::periph::uart::Uart5, ...>.rx, GpioD2, AlternateMode<PinAf8>; Undefined, Tx => Defined, Tx);
pin_impl!(TxPinExt for UartPins<drone_stm32_map::periph::uart::Uart5, ...>.tx, GpioC12, AlternateMode<PinAf8>; Tx, Undefined => Tx, Defined);

pin_impl!(RxPinExt for UartPins<drone_stm32_map::periph::uart::Usart6, ...>.rx, GpioC7, AlternateMode<PinAf8>; Undefined, Tx => Defined, Tx);
pin_impl!(RxPinExt for UartPins<drone_stm32_map::periph::uart::Usart6, ...>.rx, GpioG9, AlternateMode<PinAf8>; Undefined, Tx => Defined, Tx);
pin_impl!(TxPinExt for UartPins<drone_stm32_map::periph::uart::Usart6, ...>.tx, GpioC6, AlternateMode<PinAf8>; Tx, Undefined => Tx, Defined);
pin_impl!(TxPinExt for UartPins<drone_stm32_map::periph::uart::Usart6, ...>.tx, GpioG14, AlternateMode<PinAf8>; Tx, Undefined => Tx, Defined);

// pin_impl!(RxPinExt for UartPins<drone_stm32_map::periph::uart::Usart7, ...>.rx, GpioE7, AlternateMode<PinAf8>; Undefined, Tx => Defined, Tx);
// pin_impl!(RxPinExt for UartPins<drone_stm32_map::periph::uart::Usart7, ...>.rx, GpioF6, AlternateMode<PinAf8>; Undefined, Tx => Defined, Tx);
// pin_impl!(TxPinExt for UartPins<drone_stm32_map::periph::uart::Usart7, ...>.tx, GpioE8, AlternateMode<PinAf8>; Tx, Undefined => Tx, Defined);
// pin_impl!(TxPinExt for UartPins<drone_stm32_map::periph::uart::Usart7, ...>.tx, GpioF7, AlternateMode<PinAf8>; Tx, Undefined => Tx, Defined);

// pin_impl!(RxPinExt for UartPins<drone_stm32_map::periph::uart::Usart8, ...>.rx, GpioE0, AlternateMode<PinAf8>; Undefined, Tx => Defined, Tx);
// pin_impl!(TxPinExt for UartPins<drone_stm32_map::periph::uart::Usart8, ...>.tx, GpioE1, AlternateMode<PinAf8>; Tx, Undefined => Tx, Defined);
