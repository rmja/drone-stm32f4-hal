#![feature(prelude_import)]
#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
mod general;
mod mappings;
mod shared;

pub use self::general::{GeneralTimCfg, GeneralTimSetup, NewGeneralTimSetup};
pub use self::shared::TimFreq;

pub mod prelude {
    pub use super::{
        general::NewGeneralTimSetup,
        shared::TimFreq,
    };
}

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;
