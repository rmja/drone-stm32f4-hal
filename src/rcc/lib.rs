#![feature(proc_macro_hygiene)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod clktree;
mod diverged;
pub mod periph;
pub mod pwr;
mod rcc;

pub use self::rcc::*;
