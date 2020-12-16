#![feature(prelude_import)]
#![cfg_attr(not(feature = "std"), no_std)]

mod drv;
mod mappings;
mod sdrampins;

pub use self::drv::{FmcDrv, config};

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;
