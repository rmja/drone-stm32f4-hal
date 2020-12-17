use drone_stm32f4_hal::{fmc::config::*, rcc::clktree::*};

pub const HSECLK: HseClk = HseClk::new(8_000_000);
pub const PLLSRC_HSECLK: PllSrcMuxSignal = PllSrcMuxSignal::Hse(HSECLK);
pub const PLL: Pll = PLLSRC_HSECLK.to_pllsrc(8).to_pll(360, 2, 8);
pub const SYSCLK_PLL: SysClkMuxSignal = SysClkMuxSignal::Pll(PLL.p);
pub const SYSCLK: SysClk = SYSCLK_PLL.to_sysclk();
pub const HCLK: HClk = SYSCLK.to_hclk(1);
pub const PCLK1: PClk1 = HCLK.to_pclk1(4);
pub const PCLK2: PClk2 = HCLK.to_pclk2(2);

// Configuration and timings for the is42s16400j sdram
pub const SDRAM_CFG: SdRamCfg = SdRamCfg {
    capacity: 0x800000,
    col_bits: 8,
    row_bits: 12,
    mem_width: 16,
    bank_count: 4,
    row_count: 4096,
    cas_latency: 2,
    refresh_period_ms: 64,
    t_rcd: Timing::Ns(15),
    t_rp: Timing::Ns(15),
    t_ras_min: Timing::Ns(42),
    t_wr: Timing::MemCycles(2),
    t_rc: Timing::Ns(63),
    t_xsr: Timing::Ns(70),
    t_mrd: Timing::MemCycles(2),
    power_up_delay_us: 100,
    auto_refresh_commands: 2,
};