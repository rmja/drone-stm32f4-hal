#![feature(prelude_import)]
#![cfg_attr(not(feature = "std"), no_std)]

mod drv;
mod periph;
mod mappings;
mod sdrampins;

pub use self::drv::{FmcDrv, config};
pub use self::periph::*;

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;
