use self::traits::*;
use crate::{clktree::*, diverged::RccDiverged, periph::RccPeriph};
use drone_cortexm::{fib, reg::prelude::*, thr::prelude::*};
use drone_stm32_map::reg;
use fib::FiberFuture;

pub struct RccSetup<RccInt: IntToken> {
    /// Rcc peripheral.
    pub rcc: RccPeriph,
    /// Rcc global interrupt.
    pub rcc_int: RccInt,
}

pub struct Rcc<RccInt: IntToken> {
    rcc: RccDiverged,
    rcc_int: RccInt,
}

impl<RccInt: IntToken> Rcc<RccInt> {
    #[must_use]
    pub fn init(setup: RccSetup<RccInt>) -> Rcc<RccInt> {
        let RccSetup { rcc, rcc_int } = setup;
        Rcc {
            rcc: rcc.into(),
            rcc_int,
        }
    }
}

pub mod traits {
    use super::*;

    pub trait StabilingClkCtrl<Clk> {
        /// Enables the clock and completes the future when the clock has stabilized.
        fn enable(&self, clk: Clk) -> FiberFuture<()>;
    }

    pub trait MuxCtrl<Mux> {
        fn select<Src: MuxableSrc<Mux>>(&self, src: Src) {
            let mux = src.src_mux();
            self.select_mux(mux);
        }

        fn select_mux(&self, mux: Mux);
    }
}

impl<RccInt: IntToken> StabilingClkCtrl<HseClk> for Rcc<RccInt> {
    fn enable(&self, _clk: HseClk) -> FiberFuture<()> {
        // Enable ready interrupt.
        self.rcc.rcc_cir.modify(|r| r.set_hserdyie());

        let reg::rcc::Cir {
            hserdyc, hserdyf, ..
        } = self.rcc.rcc_cir;

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
        self.rcc.rcc_cr.modify(|r| r.set_hseon());

        // Wait for the clock to enable.
        hserdy
    }
}

impl<RccInt: IntToken> StabilingClkCtrl<Pll> for Rcc<RccInt> {
    fn enable(&self, clk: Pll) -> FiberFuture<()> {
        // Enable ready interrupt.
        self.rcc.rcc_cir.modify(|r| r.set_pllrdyie());

        let reg::rcc::Cir {
            pllrdyc, pllrdyf, ..
        } = self.rcc.rcc_cir;

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
        self.rcc.rcc_pllcfgr.modify(|r| {
            let pllp = match clk.p.div {
                2 => 0b00,
                4 => 0b01,
                6 => 0b10,
                8 => 0b11,
                _ => unreachable!(),
            };
            r.write_pllm(clk.vco.src.m)
                .write_plln(clk.vco.n)
                .write_pllp(pllp)
                .write_pllq(clk.q.div)
        });

        // Enable the clock.
        self.rcc.rcc_cr.modify(|r| r.set_pllon());

        // Wait for the clock to enable.
        pllrdy
    }
}

impl<RccInt: IntToken> MuxCtrl<PllSrcMux> for Rcc<RccInt> {
    fn select_mux(&self, mux: PllSrcMux) {
        match mux {
            PllSrcMux::Hsi(_) => self.rcc.rcc_pllcfgr.modify(|r| r.clear_pllsrc()),
            PllSrcMux::Hse(_) => self.rcc.rcc_pllcfgr.modify(|r| r.set_pllsrc()),
        }
    }
}

impl<RccInt: IntToken> MuxCtrl<SysClkMux> for Rcc<RccInt> {
    fn select_mux(&self, mux: SysClkMux) {
        let sw = match mux {
            SysClkMux::Hsi(_) => 0b00,
            SysClkMux::Hse(_) => 0b01,
            SysClkMux::Pll(_) => 0b10,
        };
        self.rcc.rcc_cfgr.modify(|r| r.write_sw(sw));
    }
}
