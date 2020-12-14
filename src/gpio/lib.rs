#![cfg_attr(not(feature = "std"), no_std)]

mod alternate;
mod head;
mod input;
mod output;
mod pin;
mod pins;

pub use self::head::GpioHead;
pub use self::pin::{GpioPin, GpioPinSpeed, OutputMode, PinPullToken, PinTypeToken};

pub mod prelude {
    pub use crate::pin::{
        AlternateMode, InputMode, NoPull, OutputMode, PinAf0, PinAf1, PinAf10, PinAf11, PinAf12,
        PinAf13, PinAf14, PinAf15, PinAf2, PinAf3, PinAf4, PinAf5, PinAf6, PinAf7, PinAf8, PinAf9,
        PinAfToken, NewPin, PinModeToken, PinPullToken, PinSpeed, PinTypeToken, PullDown, PullUp,
    };
}
