#![feature(prelude_import)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod gen;
mod gen_ch;
mod gen_cnt;
mod gen_ovf;
mod mappings;
mod shared;

pub use self::gen::{
    ConfigureTimCh1, ConfigureTimCh2, ConfigureTimCh3, ConfigureTimCh4, GeneralTimCfg,
    GeneralTimSetup, NewGeneralTimSetup,
};
pub use self::gen_ch::{IntoPinInputCaptureMode, TimChCfg};
pub use self::gen_cnt::GeneralTimCntDrv;
pub use self::gen_ovf::GeneralTimOvfDrv;
pub use self::shared::{
    ChModeToken, ChannelCaptureOverflow, DefaultLink, DirCountDown, DirCountUp, DirToken,
    DirectSelection, IndirectSelection, InputCaptureMode, LinkToken, MasterLink, OutputCompareMode,
    SelectionToken, SlaveLink, TimCh1, TimCh2, TimCh3, TimCh4, TimChToken, TimFreq, TimerCounter,
    TimerLink, TimerOverflow,
};

pub mod prelude {
    pub use super::{
        gen::{
            ConfigureTimCh1, ConfigureTimCh2, ConfigureTimCh3, ConfigureTimCh4, NewGeneralTimSetup,
        },
        gen_ch::IntoPinInputCaptureMode,
        shared::{
            ChModeToken, ChannelCaptureOverflow, DirCountDown, DirCountUp, DirToken,
            InputCaptureMode, LinkToken, OutputCompareMode, SelectionToken, TimCh1, TimCh2, TimCh3,
            TimCh4, TimChToken, TimFreq, TimerCounter, TimerLink, TimerOverflow,
        },
    };
}

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;
