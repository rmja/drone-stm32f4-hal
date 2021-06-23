use crate::{uart_setup_init, rx_drv_init, tx_drv_init, trx_drv_init, pins::{*, traits::*}};
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
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f411",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
pin_impl!(RxPinExt for UartPins<Usart2, ...>.rx, GpioD6, AlternateMode<PinAf7>; Undefined, Tx -> Defined, Tx);

pin_impl!(TxPinExt for UartPins<Usart2, ...>.tx, GpioA2, AlternateMode<PinAf7>; Tx, Undefined -> Tx, Defined);
#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f411",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
pin_impl!(TxPinExt for UartPins<Usart2, ...>.tx, GpioD5, AlternateMode<PinAf7>; Tx, Undefined -> Tx, Defined);
