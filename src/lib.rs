//! Drone OS peripheral drivers for STM32F4 micro-controllers.

#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown)]
#![no_std]

#[cfg(feature = "gpio")]
pub extern crate drone_stm32f4_gpio_drv as gpio;

#[cfg(feature = "uart")]
pub extern crate drone_stm32f4_uart_drv as uart;