#![feature(never_type)]
#![feature(prelude_import)]
#![cfg_attr(not(feature = "std"), no_std)]

mod diverged;
mod drv;
mod iface;
mod master;
mod slave;

pub use self::drv::{config, SpiDrv};
pub use self::iface::{SpiIface, traits};

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;
