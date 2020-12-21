#![feature(prelude_import)]
#![cfg_attr(not(feature = "std"), no_std)]

mod diverged;
#[macro_use]
mod drv;
mod line;
mod mappings;

pub use self::drv::{ExtiDrv, ExtiOverflow, ExtiSetup};
pub use self::line::ExtiLine;

pub mod prelude {
    pub use crate::drv::ExtiDrvLine;
}

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;
