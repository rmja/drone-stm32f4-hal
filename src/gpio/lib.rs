#![cfg_attr(not(feature = "std"), no_std)]

mod head;
mod mappings;
mod pin;
mod pins;

extern crate alloc;

pub use self::head::GpioHead;
pub use self::pin::GpioPin;
pub use self::prelude::*;

pub mod prelude {
    pub use crate::pin::{
        AlternateMode, GpioPinSpeed, InputMode, NewPin, NoPull, OutputMode, PinAf0, PinAf1,
        PinAf10, PinAf11, PinAf12, PinAf13, PinAf14, PinAf15, PinAf2, PinAf3, PinAf4, PinAf5,
        PinAf6, PinAf7, PinAf8, PinAf9, PinAfToken,
        PullDown, PullUp, PushPullType,
    };
}
