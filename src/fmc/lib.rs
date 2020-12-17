#![feature(prelude_import)]
#![cfg_attr(not(feature = "std"), no_std)]

mod drv;
mod mappings;
mod periph;
mod sdrampins;

pub use self::drv::{config, FmcDrv};
pub use self::periph::*;

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;
