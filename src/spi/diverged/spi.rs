use drone_cortexm::reg::prelude::*;
use drone_stm32_map::periph::spi::{SpiMap, SpiPeriph};

#[allow(dead_code)]
pub(crate) struct SpiDiverged<Spi: SpiMap> {
    pub(crate) rcc_busenr_spien: Spi::SRccBusenrSpien,
    pub(crate) rcc_busrstr_spirst: Spi::SRccBusrstrSpirst,
    pub(crate) rcc_bussmenr_spismen: Spi::SRccBussmenrSpismen,
    pub(crate) spi_sr: Spi::CSpiSr,
    pub(crate) spi_dr: Spi::SSpiDr,
    pub(crate) spi_cr1: Spi::SSpiCr1,
    pub(crate) spi_cr2: Spi::SSpiCr2,
    pub(crate) spi_crcpr: Spi::SSpiCrcpr,
    pub(crate) spi_rxcrcr: Spi::SSpiRxcrcr,
    pub(crate) spi_txcrcr: Spi::SSpiTxcrcr,
    pub(crate) spi_i2scfgr: Spi::SSpiI2scfgrOpt,
    pub(crate) spi_i2spr: Spi::SSpiI2sprOpt,
}

impl<Spi: SpiMap> From<SpiPeriph<Spi>> for SpiDiverged<Spi> {
    fn from(periph: SpiPeriph<Spi>) -> Self {
        let SpiPeriph {
            rcc_busenr_spien,
            rcc_busrstr_spirst,
            rcc_bussmenr_spismen,
            spi_sr,
            spi_dr,
            spi_cr1,
            spi_cr2,
            spi_crcpr,
            spi_rxcrcr,
            spi_txcrcr,
            spi_i2scfgr,
            spi_i2spr,
        } = periph;
        Self {
            rcc_busenr_spien,
            rcc_busrstr_spirst,
            rcc_bussmenr_spismen,
            spi_sr: spi_sr.into_copy(),
            spi_dr,
            spi_cr1,
            spi_cr2,
            spi_crcpr,
            spi_rxcrcr,
            spi_txcrcr,
            spi_i2scfgr,
            spi_i2spr,
        }
    }
}
