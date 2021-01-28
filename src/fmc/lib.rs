#![feature(prelude_import)]
#![cfg_attr(not(feature = "std"), no_std)]

mod drv;
mod mappings;
mod periph;
mod sdrampins;
mod setup;

pub use self::drv::FmcDrv;
pub use self::periph::*;
pub use self::{
    sdrampins::{
        FmcSdRamAddressPins,
        FmcSdRamDataPins,
        FmcSdRamPins,
        FmcSdRamBankPins,
        FmcSdRamByteMaskPins,
    }
};
pub use self::setup::{SdRamSetup, SdRamCfg};
pub use self::prelude::*;

pub mod prelude {
    pub use crate::sdrampins::traits::*;
    pub use crate::setup::Timing;
}

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;
