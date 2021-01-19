use drone_stm32f4_hal::rcc::clktree::*;

pub const HSECLK: HseClk = HseClk::new(8_000_000);
pub const PLLSRC_HSECLK: PllSrcMuxSignal = PllSrcMuxSignal::Hse(HSECLK);
pub const PLL: Pll = PLLSRC_HSECLK.to_pllsrc(8).to_pll(360, 2, 8);
pub const SYSCLK_PLL: SysClkMuxSignal = SysClkMuxSignal::Pll(PLL.p);
pub const SYSCLK: SysClk = SYSCLK_PLL.to_sysclk();
pub const HCLK: HClk = SYSCLK.to_hclk(1);
pub const PCLK1: PClk1 = HCLK.to_pclk1(4);
pub const PCLK2: PClk2 = HCLK.to_pclk2(2);

pub const TIM2_FREQ: u32 = 1_000_000;
