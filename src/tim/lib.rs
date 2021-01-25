#![feature(prelude_import)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod gen;
mod gen_ch;
mod gen_cnt;
mod gen_ovf;
mod mappings;
mod shared;
mod traits;

pub use self::gen::{
    ConfigureTimCh1, ConfigureTimCh2, ConfigureTimCh3, ConfigureTimCh4, GeneralTimCfg,
    GeneralTimSetup, NewGeneralTimSetup,
};
pub use self::gen_ch::{GeneralTimCh, GeneralTimChDrv, IntoPinInputCaptureMode};
pub use self::gen_cnt::GeneralTimCntDrv;
pub use self::gen_ovf::GeneralTimOvfDrv;
pub use self::shared::TimFreq;
pub use self::traits::*;
pub use drone_stm32_map::periph::tim::general::GeneralTimMap;

pub mod prelude {
    pub use super::{
        gen::{
            ConfigureTimCh1, ConfigureTimCh2, ConfigureTimCh3, ConfigureTimCh4, NewGeneralTimSetup,
        },
        gen_ch::IntoPinInputCaptureMode,
        shared::TimFreq,
        traits::*,
    };
}

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;
