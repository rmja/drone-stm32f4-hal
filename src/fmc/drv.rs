use crate::periph::FmcPeriph;
use core::cmp::max;
use drone_cortexm::reg::prelude::*;
use drone_stm32f4_rcc_drv::{clktree::HClk, traits::ConfiguredClk};

use self::config::*;

pub mod config {
    use drone_stm32f4_rcc_drv::{clktree::HClk, traits::ConfiguredClk};
    use crate::periph::FmcPeriph;

    pub use crate::sdrampins::*;

    #[derive(Clone, Copy, PartialEq)]
    pub enum Bank {
        Bank1,
        Bank2,
    }

    pub struct SdRamCfg {
        /// The number of column bits.
        pub col_bits: u32,
        /// The number of row bits.
        pub row_bits: u32,
        /// The memory width.
        pub mem_width: u32,
        /// The number of bank.
        pub bank_count: u32,
        /// The number of rows.
        pub row_count: u32,
        /// The refresh period.
        pub refresh_period: Timing,
        /// Row address to column address delay.
        pub t_rcd: Timing,
        /// Row precharge delay.
        pub t_rp: Timing,
        /// Min row active time.
        pub t_ras_min: Timing,
        /// Write recovery delay.
        pub t_wr: Timing,
        /// Row cycle delay.
        pub t_rc: Timing,
        /// Exit self refresh to active delay.
        pub t_xsr: Timing,
        /// Load Mode Register to active delay.
        pub t_mrd: Timing,
        /// autoRefreshCycles
        pub auto_refresh: Timing,
    }

    pub struct SdRamSetup {
        /// The fmc peripheral.
        pub fmc: FmcPeriph,
        /// The per bank sdram module configuration.
        pub bank1: Option<SdRamCfg>,
        pub bank2: Option<SdRamCfg>,
        pub clk: ConfiguredClk<HClk>,
        /// The sdram clock hclk prescaler, i.e. sdclk = hclk / sdclk_hclk_presc.
        /// Valid values are 2 and 3.
        pub sdclk_hclk_presc: u32,
    }

    impl SdRamSetup {
        pub fn for_bank2(fmc: FmcPeriph, sdram: SdRamCfg, clk: ConfiguredClk<HClk>) -> Self {
            Self {
                fmc,
                bank1: None,
                bank2: Some(sdram),
                clk,
                sdclk_hclk_presc: 2,
            }
        }
    }

    #[derive(Clone, Copy)]
    pub enum Timing {
        Ns(u32),
        Ms(u32),
        MemCycles(u32)
    }

    impl Timing {
        pub(crate) fn to_max_cycles(self, sdclk: u32) -> u32 {
            match self {
                Timing::Ns(ns) => {
                    // Round up the division:
                    // cycles = ( ns / 1000000000 ) * sdclk
                    //        = ( ns * sdclk ) / 1000000000 */
                    let ns = ns as u64;
                    let sdclk = sdclk as u64;
                    (((ns*sdclk)+1000_000_000u64-1u64)/1000_000_000u64) as u32
                },
                Timing::Ms(_ms) => unreachable!(),
                Timing::MemCycles(cycles) => cycles,
            }
        }
    }
}

pub struct FmcDrv {
    fmc: FmcPeriph,
}

impl FmcDrv {
    pub fn init_sdram<Sdcke0, Sdcke1, Sdne0, Sdne1, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15, D16, D17, D18, D19, D20, D21, D22, D23, D24, D25, D26, D27, D28, D29, D30, D31, NBL2, NBL3>(
        setup: SdRamSetup,
        pins: FmcSdRamPins<D, Sdcke0, Sdcke1, Sdne0, Sdne1, D, D, D>,
        address_pins: FmcSdRamAddressPins<D, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12>,
        data_pins: FmcSdRamDataPins<D, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15, D16, D17, D18, D19, D20, D21, D22, D23, D24, D25, D26, D27, D28, D29, D30, D31>,
        bank_pins: FmcSdRamBankPins<D, D>,
        mask_pins: FmcSdRamByteMaskPins<D, D, NBL2, NBL3>,
    ) {
        let bank1 = setup.bank1.as_ref();
        let bank2 = setup.bank2.as_ref();
        let sdclk = setup.clk.f() / setup.sdclk_hclk_presc;

        // Enable the FMC clock.
        setup.fmc.rcc_ahb3enr_fmcen.set_bit();

        // Setup banks
        // if let Some(bank1) = bank1 {

        // }
        let fmc = FmcDrv{
            fmc: setup.fmc
        };

        fmc.configure_timings(bank1, bank2, sdclk);
        
    }

    fn configure_timings(&self, bank1: Option<&SdRamCfg>, bank2: Option<&SdRamCfg>, sdclk: u32) {
        // Setup per bank timings.
        if let Some(bank1) = bank1 {
            self.fmc.fmc_sdtr1.store(|r| { r
                .write_tmrd(bank1.t_mrd.to_max_cycles(sdclk))
                .write_txsr(bank1.t_xsr.to_max_cycles(sdclk))
                .write_tras(bank1.t_ras_min.to_max_cycles(sdclk))
                .write_trcd(bank1.t_rcd.to_max_cycles(sdclk))
            })
        }
        if let Some(bank2) = bank2 {
            self.fmc.fmc_sdtr2.store(|r| { r
                .write_tmrd(bank2.t_mrd.to_max_cycles(sdclk))
                .write_txsr(bank2.t_xsr.to_max_cycles(sdclk))
                .write_tras(bank2.t_ras_min.to_max_cycles(sdclk))
                .write_trcd(bank2.t_rcd.to_max_cycles(sdclk))
            })
        }

        // Setup the timing fields that are shared between the two banks.
        let trp_slowest = max( 
            bank1.map(|b| {b.t_rp.to_max_cycles(sdclk)}).unwrap_or_default(),
            bank2.map(|b| {b.t_rp.to_max_cycles(sdclk)}).unwrap_or_default());
        let twr_slowest = max(
            bank1.map(|b| {b.t_wr.to_max_cycles(sdclk)}).unwrap_or_default(),
            bank2.map(|b| {b.t_wr.to_max_cycles(sdclk)}).unwrap_or_default());
        let trc_slowest = max(
            bank1.map(|b| {b.t_rc.to_max_cycles(sdclk)}).unwrap_or_default(),
            bank2.map(|b| {b.t_rc.to_max_cycles(sdclk)}).unwrap_or_default());

        self.fmc.fmc_sdtr1.modify(|r| { r
            .write_trp(trp_slowest)
            .write_twr(twr_slowest)
            .write_trc(trc_slowest)
        })
    }
}