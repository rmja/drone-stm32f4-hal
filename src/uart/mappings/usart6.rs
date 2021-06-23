use crate::{uart_setup_init, rx_drv_init, tx_drv_init, trx_drv_init, pins::{*, traits::*}};
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
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
pin_impl!(RxPinExt for UartPins<Usart6, ...>.rx, GpioG9, AlternateMode<PinAf8>; Undefined, Tx -> Defined, Tx);

pin_impl!(TxPinExt for UartPins<Usart6, ...>.tx, GpioC6, AlternateMode<PinAf8>; Tx, Undefined -> Tx, Defined);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
pin_impl!(TxPinExt for UartPins<Usart6, ...>.tx, GpioG14, AlternateMode<PinAf8>; Tx, Undefined -> Tx, Defined);
