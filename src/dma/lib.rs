#![feature(prelude_import)]
#![cfg_attr(not(feature = "std"), no_std)]

mod drv;

pub use self::drv::{
    config, DmaCfg, DmaChCfg, DmaStCh0, DmaStCh1, DmaStCh2, DmaStCh3, DmaStCh4, DmaStCh5, DmaStCh6,
    DmaStCh7, DmaStChToken,
};
pub use drone_stm32_map::periph::dma::DmaMap;
pub use drone_stm32_map::periph::dma::ch::DmaChMap;

pub mod prelude {
    pub use crate::drv::DmaStChToken;
}

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;
