#![feature(prelude_import)]
#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
mod general;
mod mappings;

pub use self::general::{GeneralTimCfg, config};

pub mod prelude {
    pub use crate::general::config::NewGeneralTimSetup;
}

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;
