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
        /// The cas latency.
        pub cas_latency: u32,
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
        /// Auto refresh cycles
        pub auto_refresh: Timing,
        /// An optional mode register written to the sdram during initialization.
        pub mode_register: Option<u32>,
    }

    impl SdRamCfg {
        pub(crate) fn nc(&self) -> u32 {
            match self.col_bits {
                 8 => 0b00,
                 9 => 0b01,
                10 => 0b10,
                11 => 0b11,
                _ => panic!("Unsupported number of column bits."),
            }
        }

        pub(crate) fn nr(&self) -> u32 {
            match self.row_bits {
                11 => 0b00,
                12 => 0b01,
                13 => 0b10,
                _ => panic!("Unsupported number of row bits."),
            }
        }

        pub(crate) fn mwid(&self) -> u32 {
            match self.mem_width {
                 8 => 0b00,
                16 => 0b01,
                32 => 0b10,
                _ => panic!("Unsupported memory width."),
            }
        }

        pub(crate) fn nb(&self) -> bool {
            match self.bank_count {
                2 => false,
                4 => true,
                _ => panic!("Unsupported number of banks."),
            }
        }

        pub(crate) fn sdtrt_count(&self, sdclk: u32) -> u32 {
            // From PM0090
            // Refresh rate = (SDRAM refresh rate * SDRAM clock frequency) - 20
            // SDRAM refresh rate = SDRAM refresh period / Number of rows
            // Example:
            // (64[ms]/4096[rows]) * 90[MHz] - 20
            // = 64
            
            // uint64_t refreshPeriod_x_sdRamClock = self.refresh_period. * sdclk;
        }
    }

    pub trait SdRamCfgModeRegister {
        /// Get the mode register programmed to the sdram.
        /// It typically contains burst length, burst type, latency, etc.
        fn get_mode_register(&self) -> u32;
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

enum SdRamCommand {
    NormalMode,
    ClockConfigurationEnable,
    PrechargeAll,
    AutoRefresh(u32),
    LoadModeRegister(u32),
    SelfRefresh,
    PowerDown
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
        let fmc = FmcDrv{fmc: setup.fmc};

        fmc.configure_control(bank1, bank2, setup.sdclk_hclk_presc);
        fmc.configure_timings(bank1, bank2, sdclk);

        let to_bank1 = bank1.is_some();
        let to_bank2 = bank2.is_some();
        fmc.send_command(SdRamCommand::ClockConfigurationEnable, to_bank1, to_bank2);

        // TODO: Delay 100 [us]

        fmc.send_command(SdRamCommand::PrechargeAll, to_bank1, to_bank2);

        if let Some(bank1) = bank1 {
            fmc.send_command(SdRamCommand::AutoRefresh(bank1.auto_refresh.to_max_cycles(sdclk)), true, false);
            if let Some(mrd) = bank1.mode_register {
                fmc.send_command(SdRamCommand::LoadModeRegister(mrd), true, false);
            }
        }
        if let Some(bank2) = bank2 {
            fmc.send_command(SdRamCommand::AutoRefresh(bank2.auto_refresh.to_max_cycles(sdclk)), false, true);
            if let Some(mrd) = bank2.mode_register {
                fmc.send_command(SdRamCommand::LoadModeRegister(mrd), false, true);
            }
        }

        let count_max = max( 
            bank1.map(|b| {b.sdtrt_count()}).unwrap_or_default(),
            bank2.map(|b| {b.sdtrt_count()}).unwrap_or_default());
        fmc.fmc.fmc_sdrtr.store(|r| {
            r.write_count(count_max)
            .set_cre() // Clear refresh error flag
        });
    }

    fn configure_control(&self, bank1: Option<&SdRamCfg>, bank2: Option<&SdRamCfg>, sdclk_hclk_presc: u32) {
        // Setup per bank configuration.
        if let Some(bank1) = bank1 {
            self.fmc.fmc_sdcr1.store(|r| { 
                r.write_nc(bank1.nc())
                    .write_nr(bank1.nr())
                    .write_mwid(bank1.mwid())
                    // TODO: USE NUMBER OF BANKS
                    // .write_nb(bank1.nb())
                    .write_cas(bank1.cas_latency)
                    .clear_wp() // Disable write protection
            })
        }
        if let Some(bank2) = bank2 {
            self.fmc.fmc_sdcr1.store(|r| { 
                r.write_nc(bank2.nc())
                    .write_nr(bank2.nr())
                    .write_mwid(bank2.mwid())
                    // TODO: USE NUMBER OF BANKS
                    // .write_nb(bank2.nb())
                    .write_cas(bank2.cas_latency)
                    .clear_wp() // Disable write protection
            })
        }

        // Setup shared fields.
        let sdclk = match sdclk_hclk_presc {
            2 => 0b10,
            3 => 0b11,
            _ => unreachable!(),
        };

        self.fmc.fmc_sdcr1.modify(|r| { r
            .write_sdclk(sdclk)
            .clear_rburst() // Do not use burst mode.
            .write_rpipe(0) // Read pipe is not used when not in burst mode.
        });
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

    fn send_command(&self, command: SdRamCommand, to_bank1: bool, to_bank2: bool, ) {
        self.fmc.fmc_sdcmr.store(|r| {
            let mut r = match command {
                SdRamCommand::NormalMode =>
                    r.write_mode(0b000),
                SdRamCommand::ClockConfigurationEnable =>
                    r.write_mode(0b001),
                SdRamCommand::PrechargeAll =>
                    r.write_mode(0b010),
                SdRamCommand::AutoRefresh(nrfs) =>
                    r.write_mode(0b011).write_nrfs(nrfs),
                SdRamCommand::LoadModeRegister(mrd) =>
                    r.write_mode(0b100).write_mrd(mrd),
                SdRamCommand::SelfRefresh =>
                    r.write_mode(0b101),
                SdRamCommand::PowerDown =>
                    r.write_mode(0b110),
            };
            if to_bank1 {
                r = r.set_ctb1();
            }
            if to_bank2 {
                r = r.set_ctb2();
            }
            r
        });

        loop {
            if !self.fmc.fmc_sdsr.busy.read_bit() {
                break;
            }
        }


    }
}