use crate::{clktree::*, pwr::Pwr};
use drone_cortexm::{fib, reg::prelude::*, thr::prelude::*};
use drone_stm32_map::reg;
use fib::FiberFuture;

pub struct Rcc<RccInt: IntToken> {
    /// The clock control register.
    pub rcc_cr: reg::rcc::Cr<Srt>,
    /// The PLL configuration register.
    pub rcc_pllcfgr: reg::rcc::Pllcfgr<Srt>,
    /// The clock configuration register.
    pub rcc_cfgr: reg::rcc::Cfgr<Srt>,
    /// The clock interrupt register.
    pub rcc_cir: reg::rcc::Cir<Srt>,
    /// The APB1, low-speed peripheral clock enable register.
    pub rcc_apb1enr: reg::rcc::Apb1Enr<Srt>,
    /// The APB2, high-speed peripheral clock enable register.
    pub rcc_apb2enr: reg::rcc::Apb2Enr<Srt>,
    /// The rcc global interrupt.
    pub rcc_int: RccInt,
}

pub trait StabilingClkCtrl<Clk> {
    /// Enables the clock and completes the future when the clock is stable.
    fn stabilize(&self, clk: Clk) -> FiberFuture<()>;
}

pub trait MuxCtrl<Mux> {
    fn select(&self, mux: Mux);
}

pub trait GatedClkCtrl<Clk, Gate> {
    /// Enable gated clock.
    fn enable(&self, clk: Clk, gate: Gate);
}

impl<RccInt: IntToken> StabilingClkCtrl<HseClk> for Rcc<RccInt> {
    fn stabilize(&self, clk: HseClk) -> FiberFuture<()> {
        // Enable ready interrupt.
        self.rcc_cir.modify(|r| r.set_hserdyie());

        let reg::rcc::Cir {
            hserdyc, hserdyf, ..
        } = self.rcc_cir;

        // Attach a listener that will notify us when the clock has stabilized.
        let hserdy = self.rcc_int.add_future(fib::new_fn(move || {
            if hserdyf.read_bit() {
                hserdyc.set_bit();
                fib::Complete(())
            } else {
                fib::Yielded(())
            }
        }));

        // Enable the clock.
        self.rcc_cr.modify(|r| r.set_hseon());

        // Wait for the clock to stabilize.
        hserdy
    }
}

impl<RccInt: IntToken> StabilingClkCtrl<Pll> for Rcc<RccInt> {
    fn stabilize(&self, clk: Pll) -> FiberFuture<()> {
        // Enable ready interrupt.
        self.rcc_cir.modify(|r| r.set_pllrdyie());

        let reg::rcc::Cir {
            pllrdyc, pllrdyf, ..
        } = self.rcc_cir;

        // Attach a listener that will notify us when the clock has stabilized.
        let pllrdy = self.rcc_int.add_future(fib::new_fn(move || {
            if pllrdyf.read_bit() {
                pllrdyc.set_bit();
                fib::Complete(())
            } else {
                fib::Yielded(())
            }
        }));

        // Configure the PLL.
        self.rcc_pllcfgr.modify(|r| {
            let pllp = match clk.p.div {
                2 => 0b00,
                4 => 0b01,
                6 => 0b10,
                8 => 0b11,
                _ => unreachable!()
            };
            r.write_pllm(clk.vco.src.m)
                .write_plln(clk.vco.n)
                .write_pllp(pllp)
                .write_pllq(clk.q.div)
        });

        // Enable the clock.
        self.rcc_cr.modify(|r| r.set_pllon());

        // Wait for the clock to stabilize.
        pllrdy
    }
}

impl<RccInt: IntToken> MuxCtrl<PllSrcMux> for Rcc<RccInt> {
    fn select(&self, mux: PllSrcMux) {
        match mux {
            PllSrcMux::Hsi(_) => self.rcc_pllcfgr.modify(|r| r.clear_pllsrc()),
            PllSrcMux::Hse(_) => self.rcc_pllcfgr.modify(|r| r.set_pllsrc()),
        }
    }
}

impl<RccInt: IntToken> MuxCtrl<SysClkMux> for Rcc<RccInt> {
    fn select(&self, mux: SysClkMux) {
        let sw = match mux {
            SysClkMux::Hsi(_) => 0b00,
            SysClkMux::Hse(_) => 0b01,
            SysClkMux::Pll(_) => 0b10,
        };
        self.rcc_cfgr.modify(|r| r.write_sw(sw));
    }
}

impl<RccInt: IntToken> GatedClkCtrl<PClk1, Pwr> for Rcc<RccInt> {
    fn enable(&self, clk: PClk1, gate: Pwr) {
        self.rcc_apb1enr.modify(|r| r.set_pwren());
    }
}

const HSECLK: HseClk = HseClk(8_000_000);
const PLL: Pll = PllSrcMux::Hse(HSECLK).to_pllsrc(1).to_pll(100, 10, 2);
const HCLK: HClk = SysClkMux::Pll(PLL.p).to_hclk(1);
const PCLK1: PClk1 = HCLK.to_pclk1(1);
const PCLK2: PClk2 = HCLK.to_pclk2(1);

async fn asd() {
    let rcc = Rcc {

    };
    let pwr = Pwr {
    };

    rcc.stabilize(HSECLK).await;
    rcc.select(PLL.mux());
    rcc.stabilize(PLL).await;
    rcc.enable(PCLK1, pwr);
    rcc.select(HCLK.mux());

    
}