use crate::periph::SyscfgPeriph;
use drone_cortexm::reg::prelude::*;

pub struct Syscfg {
    _syscfg: SyscfgPeriph,
}

impl Syscfg {
    #[must_use]
    pub fn with_enabled_clock(syscfg: SyscfgPeriph) -> Syscfg {
        // Enable syscfg clock.
        syscfg.rcc_apb2enr_syscfgen.set_bit();

        Syscfg { _syscfg: syscfg }
    }
}