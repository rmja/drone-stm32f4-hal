#![cfg_attr(not(feature = "std"), no_std)]

mod alternate;
mod drv;
mod input;
mod output;

pub use self::drv::{GpioPinCfg, GpioPinSpeed, OutputMode, PinPullToken, PinTypeToken};

pub mod prelude {
    pub use crate::drv::{
        AlternateMode, InputMode, NoPull, OutputMode, PinAfToken, PinPullToken, PinSpeed,
        PinTypeToken, PullDown, PullUp,
        PinAf0,
        PinAf1,
        PinAf2,
        PinAf3,
        PinAf4,
        PinAf5,
        PinAf6,
        PinAf7,
        PinAf8,
        PinAf9,
        PinAf10,
        PinAf11,
        PinAf12,
        PinAf13,
        PinAf14,
        PinAf15,
    };
}
