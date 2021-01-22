#![feature(never_type)]
#![feature(prelude_import)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod chipctrl;
mod diverged;
#[macro_use]
mod drv;
mod mappings;
mod master;
mod pins;
mod slave;

pub use self::drv::{config, SpiDrv};
pub use self::master::SpiMasterDrv;
pub use drone_stm32_map::periph::spi::SpiMap;

pub mod prelude {
    pub use crate::drv::SpiDrvInit;
}

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;
