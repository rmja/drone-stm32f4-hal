#![feature(never_type)]
#![feature(prelude_import)]
#![cfg_attr(not(feature = "std"), no_std)]

mod diverged;
mod drv;
mod rx;
mod tx;

pub use self::drv::{UartDrv, config};

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;
