use crate::{
    pins::{traits::*, *},
    rx_drv_init, trx_drv_init, tx_drv_init, uart_setup_init,
};
use drone_stm32_map::periph::gpio::pin::*;
use drone_stm32_map::periph::uart::Usart6;
use drone_stm32f4_dma_drv::DmaStCh5;
use drone_stm32f4_gpio_drv::pin_impl;
use drone_stm32f4_gpio_drv::prelude::*;
use drone_stm32f4_rcc_drv::clktree::PClk2;

uart_setup_init!(Usart6, PClk2);

rx_drv_init!(Usart6; Dma2Ch1, DmaStCh5);
rx_drv_init!(Usart6; Dma2Ch2, DmaStCh5);

tx_drv_init!(Usart6; Dma2Ch6, DmaStCh5);
tx_drv_init!(Usart6; Dma2Ch7, DmaStCh5);

trx_drv_init!(Usart6; Dma2Ch6, DmaStCh5; Dma2Ch1, DmaStCh5);
trx_drv_init!(Usart6; Dma2Ch6, DmaStCh5; Dma2Ch2, DmaStCh5);
trx_drv_init!(Usart6; Dma2Ch7, DmaStCh5; Dma2Ch1, DmaStCh5);
trx_drv_init!(Usart6; Dma2Ch7, DmaStCh5; Dma2Ch2, DmaStCh5);

pin_impl!(RxPinExt for UartPins<Usart6, ...>.rx, GpioC7, AlternateMode<PinAf8>; Undefined, Tx -> Defined, Tx);
#[cfg(any(
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f412",
    drone_stm32_map = "stm32f413",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f446",
    drone_stm32_map = "stm32f469",
))]
pin_impl!(RxPinExt for UartPins<Usart6, ...>.rx, GpioG9, AlternateMode<PinAf8>; Undefined, Tx -> Defined, Tx);

pin_impl!(TxPinExt for UartPins<Usart6, ...>.tx, GpioC6, AlternateMode<PinAf8>; Tx, Undefined -> Tx, Defined);
#[cfg(any(
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f412",
    drone_stm32_map = "stm32f413",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f446",
    drone_stm32_map = "stm32f469",
))]
pin_impl!(TxPinExt for UartPins<Usart6, ...>.tx, GpioG14, AlternateMode<PinAf8>; Tx, Undefined -> Tx, Defined);
