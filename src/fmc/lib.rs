#![feature(prelude_import)]
#![cfg_attr(not(feature = "std"), no_std)]

mod drv;
mod mappings;
mod periph;
mod sdrampins;
mod setup;

pub use self::drv::FmcDrv;
pub use self::periph::*;
pub use self::prelude::*;
pub use self::sdrampins::{
    FmcSdRamAddressPins, FmcSdRamBankPins, FmcSdRamByteMaskPins, FmcSdRamDataPins, FmcSdRamPins,
};
pub use self::setup::{SdRamCfg, SdRamSetup};

pub mod prelude {
    pub use crate::sdrampins::traits::*;
    pub use crate::setup::Timing;
}

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;
