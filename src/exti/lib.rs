#![feature(prelude_import)]
#![cfg_attr(not(feature = "std"), no_std)]

mod diverged;
mod drv;
mod line;
mod mappings;
mod periph;
mod syscfg;

extern crate alloc;

pub use self::drv::ExtiDrv;
pub use self::line::{ExtiLine, ExtiOverflow};
pub use self::periph::*;
pub use self::prelude::*;
pub use self::syscfg::Syscfg;
use drone_stm32_map::periph::exti::{
    ExtiFtsrFt, ExtiPrPif, ExtiRtsrRt, ExtiSwierSwi, SyscfgExticrExti,
};

/// A redefinition of the `ExtiMap` from the `drone-stm32-map` crate with forced
/// availability of required registers.
pub trait ExtiMap:
    drone_stm32_map::periph::exti::ExtiMap
    + SyscfgExticrExti
    + ExtiRtsrRt
    + ExtiFtsrFt
    + ExtiSwierSwi
    + ExtiPrPif
{
}

pub mod prelude {
    pub use crate::drv::ExtiDrvLine;
    pub use crate::drv::{BothEdges, EdgeMap, FallingEdge, NoEdge, RisingEdge};
}

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;
