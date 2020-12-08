use crate::periph::RccPeriph;
use drone_cortexm::reg::prelude::*;
use drone_stm32_map::reg;

#[allow(dead_code)]
pub(crate) struct RccDiverged {
    pub(crate) rcc_cr: reg::rcc::Cr<Srt>,
    pub(crate) rcc_pllcfgr: reg::rcc::Pllcfgr<Srt>,
    pub(crate) rcc_cfgr: reg::rcc::Cfgr<Srt>,
    pub(crate) rcc_cir: reg::rcc::Cir<Crt>,
}

impl From<RccPeriph> for RccDiverged {
    fn from(periph: RccPeriph) -> Self {
        let RccPeriph {
            rcc_cr,
            rcc_pllcfgr,
            rcc_cfgr,
            rcc_cir,
        } = periph;
        Self {
            rcc_cr,
            rcc_pllcfgr,
            rcc_cfgr,
            rcc_cir: rcc_cir.into_copy(),
        }
    }
}
