use crate::{setup::*, periph::FmcPeriph};
use core::cmp::max;
use drone_cortexm::reg::prelude::*;
use drone_core::bitfield::Bitfield;

pub struct FmcDrv {
    fmc: FmcPeriph,
    bank1_capacity: Option<usize>,
    bank2_capacity: Option<usize>,
}

#[allow(dead_code)]
enum SdRamCommand {
    NormalMode,
    ClockConfigurationEnable,
    PrechargeAll,
    AutoRefresh(u32),
    LoadModeRegister(u32),
    SelfRefresh,
    PowerDown,
}

#[derive(Clone, Copy, Bitfield)]
#[bitfield(
    burst_length(rw, 0, 3),
    burst_type(rw, 3, 1),
    latency_mode(rw, 4, 3),
    operating_mode(rw, 7, 2),
    write_burst_mode(rw, 9, 1)
)]
struct SdRamModeRegister(u32);

impl FmcDrv {
    pub fn init_sdram<
        Sdcke0: Opt,
        Sdcke1: Opt,
        Sdne0: Opt,
        Sdne1: Opt,
        A11: Opt,
        A12: Opt,
        D8: Opt,
        D9: Opt,
        D10: Opt,
        D11: Opt,
        D12: Opt,
        D13: Opt,
        D14: Opt,
        D15: Opt,
        D16: Opt,
        D17: Opt,
        D18: Opt,
        D19: Opt,
        D20: Opt,
        D21: Opt,
        D22: Opt,
        D23: Opt,
        D24: Opt,
        D25: Opt,
        D26: Opt,
        D27: Opt,
        D28: Opt,
        D29: Opt,
        D30: Opt,
        D31: Opt,
        BA1: Opt,
        NBL2,
        NBL3,
    >(
        setup: SdRamSetup,
        _pins: FmcSdRamPins<D, Sdcke0, Sdcke1, Sdne0, Sdne1, D, D, D>,
        _address_pins: FmcSdRamAddressPins<D, D, D, D, D, D, D, D, D, D, D, A11, A12>,
        _data_pins: FmcSdRamDataPins<
            D,
            D,
            D,
            D,
            D,
            D,
            D,
            D,
            D8,
            D9,
            D10,
            D11,
            D12,
            D13,
            D14,
            D15,
            D16,
            D17,
            D18,
            D19,
            D20,
            D21,
            D22,
            D23,
            D24,
            D25,
            D26,
            D27,
            D28,
            D29,
            D30,
            D31,
        >,
        _bank_pins: FmcSdRamBankPins<D, BA1>,
        _mask_pins: FmcSdRamByteMaskPins<D, D, NBL2, NBL3>,
    ) -> Self {
        let bank1 = setup.bank1.as_ref();
        let bank2 = setup.bank2.as_ref();
        let sdclk = setup.clk.f() / setup.sdclk_hclk_presc;

        // Verify pin configuration.
        if bank1.is_some() {
            assert_eq!(
                1,
                Sdcke0::NUM,
                "The SDCKE pin is not configured for bank 1"
            );
            assert_eq!(1, Sdne0::NUM, "The SDNE pin is not configured for bank 1");
        }
        if bank2.is_some() {
            assert_eq!(
                1,
                Sdcke1::NUM,
                "The SDCKE pin is not configured for bank 2"
            );
            assert_eq!(1, Sdne1::NUM, "The SDNE pin is not configured for bank 2");
        }

        let max_row_bits = max(
            bank1.map(|b| b.row_bits).unwrap_or_default(),
            bank2.map(|b| b.row_bits).unwrap_or_default(),
        );
        let defined_address_pins = 11 + A11::NUM + A12::NUM;
        assert_eq!(
            defined_address_pins, max_row_bits,
            "The number of sdram row bits does not match the number of A pins"
        );

        let max_mem_width = max(
            bank1.map(|b| b.mem_width).unwrap_or_default(),
            bank2.map(|b| b.mem_width).unwrap_or_default(),
        );
        let defined_data_pins = 8
            + D8::NUM
            + D9::NUM
            + D10::NUM
            + D11::NUM
            + D12::NUM
            + D13::NUM
            + D14::NUM
            + D15::NUM
            + D16::NUM
            + D17::NUM
            + D18::NUM
            + D19::NUM
            + D20::NUM
            + D21::NUM
            + D22::NUM
            + D23::NUM
            + D24::NUM
            + D25::NUM
            + D26::NUM
            + D27::NUM
            + D28::NUM
            + D29::NUM
            + D30::NUM
            + D31::NUM;
        assert_eq!(
            defined_data_pins, max_mem_width,
            "The number memory width of the sdram does not match the number of D pins"
        );

        let max_bank_count = max(
            bank1.map(|b| b.bank_count).unwrap_or_default(),
            bank2.map(|b| b.bank_count).unwrap_or_default(),
        );
        let defined_bank_pins = 1 + BA1::NUM;
        assert_eq!(
            2 * defined_bank_pins,
            max_bank_count,
            "The number of sdram internal banks does not match the number of BA pins"
        );

        // Enable the FMC clock.
        setup.fmc.rcc_ahb3enr_fmcen.set_bit();

        // Setup banks
        let fmc = FmcDrv {
            fmc: setup.fmc,
            bank1_capacity: bank1.map(|b| b.capacity),
            bank2_capacity: bank2.map(|b| b.capacity),
        };

        fmc.configure_control(bank1, bank2, setup.sdclk_hclk_presc);
        fmc.configure_timings(bank1, bank2, sdclk);

        let to_bank1 = bank1.is_some();
        let to_bank2 = bank2.is_some();

        // The SDRAM power up sequence is as follows (JEDEC Standard 21C 3.11.5.4):
        // 1. Apply power and start clock. Attempt to maintain a NOP condition at the inputs
        // 2. Maintain stable power, stable clock, and a NOP condition for a minimum of 200us
        // 3. Issue precharge commands for all banks of the device
        // 4. Issue 8 or more auto-refresh commands
        // 5. Issue a mode register set command to initialize the mode register

        // The SDRAM initialization sequence is outlined in PM0090 §37.7.3.

        // Start delivering the clock to the memory.
        fmc.send_command(SdRamCommand::ClockConfigurationEnable, to_bank1, to_bank2);

        // Wait the prescribed power-up delay.
        // let power_up_delay_us_max = max(
        //     bank1.map(|b| b.power_up_delay_us).unwrap_or_default(),
        //     bank2.map(|b| b.power_up_delay_us).unwrap_or_default(),
        // );
        // TODO: Delay power_up_delay_us_max

        // Issue Precharge-All command to banks.
        fmc.send_command(SdRamCommand::PrechargeAll, to_bank1, to_bank2);

        // Issue the required number of Auto-Refresh commands.
        let auto_refresh_commands_max = max(
            bank1.map(|b| b.auto_refresh_commands).unwrap_or_default(),
            bank2.map(|b| b.auto_refresh_commands).unwrap_or_default(),
        );
        fmc.send_command(
            SdRamCommand::AutoRefresh(auto_refresh_commands_max),
            to_bank1,
            to_bank2,
        );

        // Issue the Load-Module-Register command.
        if let Some(bank1) = bank1 {
            let mode_register = FmcDrv::get_mode_register(bank1.cas_latency);
            fmc.send_command(SdRamCommand::LoadModeRegister(mode_register.0), true, false);
        }
        if let Some(bank2) = bank2 {
            let mode_register = FmcDrv::get_mode_register(bank2.cas_latency);
            fmc.send_command(SdRamCommand::LoadModeRegister(mode_register.0), false, true);
        }

        // Program the refresh rate.
        let count_max = max(
            bank1.map(|b| b.sdtrt_count(sdclk)).unwrap_or_default(),
            bank2.map(|b| b.sdtrt_count(sdclk)).unwrap_or_default(),
        );
        fmc.fmc.fmc_sdrtr.store(|r| {
            r.write_count(count_max).set_cre() // Clear refresh error flag
        });

        fmc
    }

    fn get_mode_register(cas_latency: u32) -> SdRamModeRegister {
        let mut mode_register = SdRamModeRegister(0);
        mode_register
            .write_burst_length(0) // Must be 0, see PM0090 §37.7.3.
            .clear_burst_type() // 0: Sequential
            .write_latency_mode(cas_latency)
            .write_operating_mode(0) // 0: Standard operation
            .set_write_burst_mode(); // 1: Single location access
        mode_register
    }

    fn configure_control(
        &self,
        bank1: Option<&SdRamCfg>,
        bank2: Option<&SdRamCfg>,
        sdclk_hclk_presc: u32,
    ) {
        // Setup per bank configuration.
        if let Some(bank1) = bank1 {
            self.fmc.fmc_sdcr1.store(|r| {
                r.write_nc(bank1.sdcr_nc()) // Number of column address bits
                    .write_nr(bank1.sdcr_nr()) // Number of row address bits
                    .write_mwid(bank1.sdcr_mwid()) // Memory data bus width
                    .write_nb(bank1.sdcr_nb()) // Number of internal banks
                    .write_cas(bank1.cas_latency) // CAS latency
                    .clear_wp() // Disable write protection
                                // SDCLK is shared, see below.
                                // RBURST is shared, see below.
                                // RPIPE is shared, see below.
            });
        }
        if let Some(bank2) = bank2 {
            self.fmc.fmc_sdcr2.store(|r| {
                r.write_nc(bank2.sdcr_nc()) // Number of column address bits
                    .write_nr(bank2.sdcr_nr()) // Number of row address bits
                    .write_mwid(bank2.sdcr_mwid()) // Memory data bus width
                    .write_nb(bank2.sdcr_nb()) // Number of internal banks
                    .write_cas(bank2.cas_latency) // CAS latency
                    .clear_wp() // Disable write protection
                                // SDCLK is read only.
                                // RBURST is don't care.
                                // RPIPE is read only.
            });
        }

        // Setup shared fields.
        self.fmc.fmc_sdcr1.modify(|r| {
            r.write_sdclk(match sdclk_hclk_presc {
                2 => 0b10,
                3 => 0b11,
                _ => unreachable!(),
            })
            .clear_rburst() // Do not use burst mode
            .write_rpipe(0b01) // One HCLK clock cycle delay for reading data after CAS latency
        });
    }

    fn configure_timings(&self, bank1: Option<&SdRamCfg>, bank2: Option<&SdRamCfg>, sdclk: u32) {
        let twr_slowest = max(
            bank1
                .map(|b| b.t_wr.to_max_cycles(sdclk))
                .unwrap_or_default(),
            bank2
                .map(|b| b.t_wr.to_max_cycles(sdclk))
                .unwrap_or_default(),
        );

        // Setup per bank timings.
        if let Some(bank1) = bank1 {
            self.fmc.fmc_sdtr1.store(|r| {
                r.write_tmrd(bank1.t_mrd.to_max_cycles(sdclk)) // Load Mode Register to Active
                    .write_txsr(bank1.t_xsr.to_max_cycles(sdclk)) // Exit Self-refresh delay
                    .write_tras(bank1.t_ras_min.to_max_cycles(sdclk)) // Self refresh time
                    // TRC is shared, see below.
                    .write_twr(twr_slowest) // Recovery delay - must be written to both banks to the slowest value
                    // TRP is shared, see below.
                    .write_trcd(bank1.t_rcd.to_max_cycles(sdclk)) // Row to column delay
            });
        }
        if let Some(bank2) = bank2 {
            self.fmc.fmc_sdtr2.store(|r| {
                r.write_tmrd(bank2.t_mrd.to_max_cycles(sdclk)) // Load Mode Register to Active
                    .write_txsr(bank2.t_xsr.to_max_cycles(sdclk)) // Exit Self-refresh delay
                    .write_tras(bank2.t_ras_min.to_max_cycles(sdclk)) // Self refresh time
                    // TRC is don't care.
                    .write_twr(twr_slowest) // Recovery delay - must be written to both banks to the slowest value
                    // TRP is don't care.
                    .write_trcd(bank2.t_rcd.to_max_cycles(sdclk))
            });
        }

        // Setup shared timing fields.
        let trp_slowest = max(
            bank1
                .map(|b| b.t_rp.to_max_cycles(sdclk))
                .unwrap_or_default(),
            bank2
                .map(|b| b.t_rp.to_max_cycles(sdclk))
                .unwrap_or_default(),
        );
        let trc_slowest = max(
            bank1
                .map(|b| b.t_rc.to_max_cycles(sdclk))
                .unwrap_or_default(),
            bank2
                .map(|b| b.t_rc.to_max_cycles(sdclk))
                .unwrap_or_default(),
        );

        self.fmc.fmc_sdtr1.modify(|r| {
            r.write_trp(trp_slowest)
                .write_twr(twr_slowest)
                .write_trc(trc_slowest)
        });
    }

    fn send_command(&self, command: SdRamCommand, to_bank1: bool, to_bank2: bool) {
        // Write the command to the command register.
        self.fmc.fmc_sdcmr.store(|r| {
            let mut r = match command {
                SdRamCommand::NormalMode => r.write_mode(0b000),
                SdRamCommand::ClockConfigurationEnable => r.write_mode(0b001),
                SdRamCommand::PrechargeAll => r.write_mode(0b010),
                SdRamCommand::AutoRefresh(nrfs) => r.write_mode(0b011).write_nrfs(nrfs),
                SdRamCommand::LoadModeRegister(mrd) => {
                    assert_eq!(mrd, mrd & 0x1FFF);
                    r.write_mode(0b100).write_mrd(mrd)
                }
                SdRamCommand::SelfRefresh => r.write_mode(0b101),
                SdRamCommand::PowerDown => r.write_mode(0b110),
            };
            if to_bank1 {
                r = r.set_ctb1();
            }
            if to_bank2 {
                r = r.set_ctb2();
            }
            r
        });

        // Wait for the controller to complete the command.
        loop {
            if !self.fmc.fmc_sdsr.busy.read_bit() {
                break;
            }
        }
    }

    pub fn bank1_slice<'a, T: Sized>(&self) -> &'a mut [T] {
        FmcDrv::slice(0xC000_0000, self.bank1_capacity.unwrap())
    }

    pub fn bank2_slice<'a, T: Sized>(&self) -> &'a mut [T] {
        FmcDrv::slice(0xD000_0000, self.bank2_capacity.unwrap())
    }

    fn slice<'a, T: Sized>(base_address: u32, capacity: usize) -> &'a mut [T] {
        // Memory bank base addresses are in PM0090 figure 457: FMC memory banks.
        let sizeof_t = core::mem::size_of::<T>();
        unsafe { core::slice::from_raw_parts_mut(base_address as *mut T, capacity / sizeof_t) }
    }
}

trait FmcSdcrExt {
    fn write_nb(&mut self, val: bool) -> &mut Self;
}

impl FmcSdcrExt for drone_stm32_map::reg::fmc::sdcr1::Hold<'_, drone_cortexm::reg::prelude::Srt> {
    fn write_nb(&mut self, val: bool) -> &mut Self {
        if val {
            self.set_nb()
        } else {
            self.clear_nb()
        }
    }
}

impl FmcSdcrExt for drone_stm32_map::reg::fmc::sdcr2::Hold<'_, drone_cortexm::reg::prelude::Srt> {
    fn write_nb(&mut self, val: bool) -> &mut Self {
        if val {
            self.set_nb()
        } else {
            self.clear_nb()
        }
    }
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
                let cycles = ((ns * sdclk) + 1_000_000_000_u64 - 1u64) / 1_000_000_000_u64;
                cycles as u32
            }
            Timing::MemCycles(cycles) => cycles,
        }
    }
}

impl SdRamCfg {
    pub(crate) fn sdcr_nc(&self) -> u32 {
        match self.col_bits {
            8 => 0b00,
            9 => 0b01,
            10 => 0b10,
            11 => 0b11,
            _ => panic!("Unsupported number of column bits."),
        }
    }

    pub(crate) fn sdcr_nr(&self) -> u32 {
        match self.row_bits {
            11 => 0b00,
            12 => 0b01,
            13 => 0b10,
            _ => panic!("Unsupported number of row bits."),
        }
    }

    pub(crate) fn sdcr_mwid(&self) -> u32 {
        match self.mem_width {
            8 => 0b00,
            16 => 0b01,
            32 => 0b10,
            _ => panic!("Unsupported memory width."),
        }
    }

    pub(crate) fn sdcr_nb(&self) -> bool {
        match self.bank_count {
            2 => false,
            4 => true,
            _ => panic!("Unsupported number of banks."),
        }
    }

    pub(crate) fn sdtrt_count(&self, sdclk: u32) -> u32 {
        // From PM0090:
        // Refresh rate = (SDRAM refresh rate * SDRAM clock frequency) - 20
        // where: SDRAM refresh rate = SDRAM refresh period / Number of rows
        // Example:
        // (64[ms]/4096[rows]) * 90[MHz] - 20
        // = 0.0015625[ms] * 90[MHz] - 20
        // = 15.625[us] * 90[MHz] - 20
        // = 1406.25 - 20 ~= 1386
        // Or equivalently for better precision:
        // = (64[ms] * 90[MHz]) / 4096[rows] - 20
        // = (64[ms] * 90000[kHz]) / 4096[rows] - 20
        // = 5760000 / 4096 - 20
        let sdclk_khz = sdclk / 1_000;
        (self.refresh_period_ms * sdclk_khz) / self.row_count - 20
    }
}
