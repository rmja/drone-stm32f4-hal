#![feature(prelude_import)]
#![cfg_attr(not(feature = "std"), no_std)]

mod drv;

pub use self::drv::{
    DmaCfg, DmaChCfg, DmaChSetup, DmaChSetupInit, DmaStCh0, DmaStCh1, DmaStCh2, DmaStCh3, DmaStCh4,
    DmaStCh5, DmaStCh6, DmaStCh7, DmaStChToken,
};

pub mod prelude {
    pub use crate::drv::{DmaChSetupInit, DmaStChToken};
}

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;