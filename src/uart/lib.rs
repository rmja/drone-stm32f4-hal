#![feature(never_type)]
#![feature(prelude_import)]
#![cfg_attr(not(feature = "std"), no_std)]

mod diverged;
mod drv;
mod mappings;
mod pins;
mod rx;
mod setup;
mod tx;

extern crate alloc;

pub use self::drv::UartDrv;
pub use self::pins::UartPins;
pub use self::prelude::*;
pub use self::rx::UartRxDrv;
pub use self::setup::{BaudRate, Parity, StopBits, UartSetup};
pub use self::tx::UartTxDrv;
pub use drone_stm32_map::periph::uart::UartMap;

pub mod prelude {
    pub use crate::drv::{IntoRxDrv, IntoTrxDrv, IntoTxDrv};
    pub use crate::pins::traits::*;
    pub use crate::setup::UartSetupInit;
}

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;
