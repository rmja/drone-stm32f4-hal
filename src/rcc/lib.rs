#![feature(const_panic)]
#![feature(proc_macro_hygiene)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod clktree;
mod diverged;
mod flash;
mod periph;
mod pwr;
mod rcc;
mod traits;

pub use self::flash::Flash;
pub use self::periph::*;
pub use self::pwr::Pwr;
pub use self::rcc::{Rcc, RccSetup};
pub use self::prelude::*;

pub mod prelude {
    pub use crate::traits::*;
}
