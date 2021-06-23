//! Drone OS peripheral drivers for STM32F4 micro-controllers.

#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown)]
#![no_std]

pub use drone_cortexm::thr::IntToken;

pub mod dwt;

#[cfg(feature = "dma")]
pub extern crate drone_stm32f4_dma_drv as dma;

#[cfg(feature = "exti")]
pub extern crate drone_stm32f4_exti_drv as exti;

#[cfg(feature = "fmc")]
pub extern crate drone_stm32f4_fmc_drv as fmc;

#[cfg(feature = "gpio")]
pub extern crate drone_stm32f4_gpio_drv as gpio;

#[cfg(feature = "rcc")]
pub extern crate drone_stm32f4_rcc_drv as rcc;

#[cfg(feature = "spi")]
pub extern crate drone_stm32f4_spi_drv as spi;

#[cfg(feature = "tim")]
pub extern crate drone_stm32f4_tim_drv as tim;

#[cfg(feature = "uart")]
pub extern crate drone_stm32f4_uart_drv as uart;

pub mod prelude {
    #[cfg(feature = "dma")]
    pub use crate::dma::prelude::*;

    #[cfg(feature = "exti")]
    pub use crate::exti::prelude::*;

    #[cfg(feature = "fmc")]
    pub use crate::fmc::prelude::*;

    #[cfg(feature = "gpio")]
    pub use crate::gpio::prelude::*;

    #[cfg(feature = "rcc")]
    pub use crate::rcc::prelude::*;

    #[cfg(feature = "spi")]
    pub use crate::spi::prelude::*;

    #[cfg(feature = "tim")]
    pub use crate::tim::prelude::*;

    #[cfg(feature = "uart")]
    pub use crate::uart::prelude::*;
}