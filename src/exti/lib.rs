#![feature(prelude_import)]
#![cfg_attr(not(feature = "std"), no_std)]

mod diverged;
#[macro_use]
mod drv;
mod line;
mod mappings;
mod periph;
mod syscfg;

pub use self::drv::{ExtiDrv, ExtiSetup};
pub use self::line::{ExtiLine, ExtiOverflow};
pub use self::periph::*;
pub use self::syscfg::Syscfg;

pub mod prelude {
    pub use crate::drv::ExtiDrvLine;
}

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;
