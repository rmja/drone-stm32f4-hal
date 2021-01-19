#![feature(associated_type_defaults)]
#![feature(prelude_import)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[macro_use]
mod gen;
#[macro_use]
mod gen_ch;
mod mappings;
mod shared;

pub use self::gen::{
    ConfigureTimCh1, ConfigureTimCh2, ConfigureTimCh3, ConfigureTimCh4, DefaultLink, DirToken,
    GeneralTimCfg, GeneralTimSetup, LinkToken, MasterLink, NewGeneralTimSetup, SlaveLink,
    GeneralTimerLink,
};
pub use self::gen_ch::{
    ChModeToken, DirectSelection, IndirectSelection, InputCaptureMode, IntoPinInputCaptureMode,
    OutputCompareMode, TimCh1, TimCh2, TimCh3, TimCh4, TimChCfg, TimChToken,
};
pub use self::shared::TimFreq;

pub mod prelude {
    pub use super::{
        gen::{
            ConfigureTimCh1, ConfigureTimCh2, ConfigureTimCh3, ConfigureTimCh4, DirToken,
            LinkToken, NewGeneralTimSetup, GeneralTimerLink,
        },
        gen_ch::{
            ChModeToken, IntoPinInputCaptureMode, TimCh1, TimCh2, TimCh3, TimCh4, TimChToken,
        },
        shared::TimFreq,
    };
}

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;
