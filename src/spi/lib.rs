#![feature(never_type)]
#![feature(prelude_import)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod chipctrl;
mod diverged;
mod drv;
mod mappings;
mod master;
mod pins;
mod setup;

pub use self::drv::SpiDrv;
pub use self::master::SpiMasterDrv;
pub use self::pins::SpiPins;
pub use self::prelude::*;
pub use self::setup::{BaudRate, ClkPol, FirstBit, Prescaler, SpiSetup};
pub use drone_stm32_map::periph::spi::SpiMap;

pub mod prelude {
    pub use super::setup::NewSpiSetup;
    pub use crate::drv::IntoMaster;
    pub use crate::pins::traits::*;
}

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;
