#![feature(prelude_import)]
#![cfg_attr(not(feature = "std"), no_std)]

mod diverged;
mod drv;
mod mappings;

pub use self::drv::{ExtiDrv, ExtiOverflow, ExtiSetup};

pub mod prelude {
    pub use crate::drv::ExtiLine;
}

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;
