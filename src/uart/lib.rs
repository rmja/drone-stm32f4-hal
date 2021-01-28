#![feature(never_type)]
#![feature(prelude_import)]
#![cfg_attr(not(feature = "std"), no_std)]

mod diverged;
mod drv;
mod mappings;
mod pins;
mod rx;
mod tx;
mod setup;

pub use self::drv::UartDrv;
pub use self::prelude::*;
pub use self::setup::UartSetup;
pub use drone_stm32_map::periph::uart::UartMap;
pub use self::pins::UartPins;

pub mod prelude {
    pub use crate::drv::{UartRxDrvInit, UartTxDrvInit};
    pub use crate::setup::{UartSetupInit, BaudRate, Parity, StopBits};
    pub use crate::pins::traits::*;
}

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;
