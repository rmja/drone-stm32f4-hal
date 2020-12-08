#![feature(proc_macro_hygiene)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod clktree;
mod diverged;
mod flash;
mod periph;
mod pwr;
mod rcc;

pub use self::flash::Flash;
pub use self::periph::*;
pub use self::pwr::Pwr;
pub use self::rcc::{Rcc, RccSetup};

pub mod traits {
    pub use crate::flash::traits::*;
    pub use crate::pwr::traits::*;
    pub use crate::rcc::traits::*;
}
