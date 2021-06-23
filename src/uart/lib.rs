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

extern crate alloc;

pub use self::drv::UartDrv;
pub use self::prelude::*;
pub use self::setup::{UartSetup, BaudRate, Parity, StopBits};
pub use drone_stm32_map::periph::uart::UartMap;
pub use self::pins::UartPins;
pub use self::tx::UartTxDrv;
pub use self::rx::UartRxDrv;

pub mod prelude {
    pub use crate::drv::{IntoRxDrv, IntoTxDrv, IntoTrxDrv};
    pub use crate::setup::UartSetupInit;
    pub use crate::pins::traits::*;
}

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;
