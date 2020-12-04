#![feature(never_type)]
#![feature(prelude_import)]
#![cfg_attr(not(feature = "std"), no_std)]

mod drv;
mod master;
mod slave;

pub use self::drv::{config, SpiDrv};

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;
