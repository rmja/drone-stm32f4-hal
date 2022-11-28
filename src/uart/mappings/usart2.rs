use crate::{
    pins::{traits::*, *},
    rx_drv_init, trx_drv_init, tx_drv_init, uart_setup_init,
};
use drone_stm32_map::periph::gpio::pin::*;
use drone_stm32_map::periph::uart::Usart2;
use drone_stm32f4_dma_drv::DmaStCh4;
use drone_stm32f4_gpio_drv::pin_impl;
use drone_stm32f4_gpio_drv::prelude::*;
use drone_stm32f4_rcc_drv::clktree::PClk1;

uart_setup_init!(Usart2, PClk1);

rx_drv_init!(Usart2; Dma1Ch5, DmaStCh4);

tx_drv_init!(Usart2; Dma1Ch6, DmaStCh4);

trx_drv_init!(Usart2; Dma1Ch6, DmaStCh4; Dma1Ch5, DmaStCh4);

pin_impl!(RxPinExt for UartPins<Usart2, ...>.rx, GpioA3, AlternateMode<PinAf7>; Undefined, Tx -> Defined, Tx);
#[cfg(any(
    drone_stm32_map = "stm32f401",
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f411",
    drone_stm32_map = "stm32f412",
    drone_stm32_map = "stm32f413",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f446",
    drone_stm32_map = "stm32f469",
))]
pin_impl!(RxPinExt for UartPins<Usart2, ...>.rx, GpioD6, AlternateMode<PinAf7>; Undefined, Tx -> Defined, Tx);

pin_impl!(TxPinExt for UartPins<Usart2, ...>.tx, GpioA2, AlternateMode<PinAf7>; Tx, Undefined -> Tx, Defined);
#[cfg(any(
    drone_stm32_map = "stm32f401",
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f411",
    drone_stm32_map = "stm32f412",
    drone_stm32_map = "stm32f413",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f446",
    drone_stm32_map = "stm32f469",
))]
pin_impl!(TxPinExt for UartPins<Usart2, ...>.tx, GpioD5, AlternateMode<PinAf7>; Tx, Undefined -> Tx, Defined);
