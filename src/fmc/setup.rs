use crate::periph::FmcPeriph;
use drone_stm32f4_rcc_drv::{clktree::HClk, ConfiguredClk};

pub use crate::sdrampins::*;

pub struct SdRamSetup {
    /// The fmc peripheral.
    pub fmc: FmcPeriph,
    /// Sd-ram module configuration for bank 1, mapped to memory address 0xC0000000....
    pub bank1: Option<SdRamCfg>,
    /// Sd-ram module configuration for bank 2, mapped to memory address 0xD0000000....
    pub bank2: Option<SdRamCfg>,
    pub clk: ConfiguredClk<HClk>,
    /// The sdram clock hclk prescaler, i.e. sdclk = hclk / sdclk_hclk_presc.
    /// Valid values are 2 and 3.
    pub sdclk_hclk_presc: u32,
}

impl SdRamSetup {
    pub fn new_bank2(fmc: FmcPeriph, sdram: SdRamCfg, clk: ConfiguredClk<HClk>) -> Self {
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
    MemCycles(u32),
}

pub struct SdRamCfg {
    /// The capacity in bytes.
    pub capacity: usize,
    /// The number of column bits.
    pub col_bits: u32,
    /// The number of row bits, 11: A0-A10, 12: A0-A11, 13: A0-A12.
    pub row_bits: u32,
    /// The memory width, 8: D0-D7, 16: D0-D15, 32: D0-D31.
    pub mem_width: u32,
    /// The number of banks inside the sdram, must be 2 or 4.
    pub bank_count: u32,
    /// The number of rows.
    pub row_count: u32,
    /// The number of cas latency sdram clock cycles.
    pub cas_latency: u32,
    /// The refresh period in milliseconds.
    pub refresh_period_ms: u32,
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
    /// The power up delay in microseconds, see the initialization section in the datasheet. It is typically 100us.
    pub power_up_delay_us: u32,
    /// The number of auto refresh commands that must be sent during initialization, see the initialization section in the datasheet. It is typically 8.
    pub auto_refresh_commands: u32,
}
