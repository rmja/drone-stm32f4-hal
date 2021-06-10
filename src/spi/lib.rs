#![feature(never_type)]
#![feature(prelude_import)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod chipctrl;
mod diverged;
mod drv;
mod mappings;
mod master;
mod setup;
mod pins;

pub use self::drv::SpiDrv;
pub use self::master::SpiMasterDrv;
pub use self::setup::{SpiSetup, BaudRate, Prescaler, ClkPol, FirstBit};
pub use drone_stm32_map::periph::spi::SpiMap;
pub use self::prelude::*;
pub use self::pins::SpiPins;

pub mod prelude {
    pub use crate::drv::IntoMaster;
    pub use super::setup::NewSpiSetup;
    pub use crate::pins::traits::*;
}

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;
