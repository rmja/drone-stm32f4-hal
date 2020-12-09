use self::traits::*;
use crate::{clktree::*, diverged::RccDiverged, periph::RccPeriph};
use drone_cortexm::{fib, reg::prelude::*, thr::prelude::*};
use drone_stm32_map::reg;
use fib::FiberFuture;

/// Rcc controller setup.
pub struct RccSetup<RccInt: IntToken> {
    /// Rcc peripheral.
    pub rcc: RccPeriph,
    /// Rcc global interrupt.
    pub rcc_int: RccInt,
}

/// Rcc controller.
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

    pub trait ClkCtrl<Clk> {
        /// Configures the clock `clk`.
        fn configure(&self, clk: Clk);
    }

    pub trait StabilingClkCtrl<Clk> {
        /// Enables the clock `clk  and completes the future when the clock has stabilized.
        fn stabilize(&self, clk: Clk) -> FiberFuture<()>;
    }

    pub trait MuxCtrl<MuxSignal> {
        /// Selects the source signal of the associated mux.
        fn select(&self, signal: MuxSignal);
    }
}

impl<RccInt: IntToken> StabilingClkCtrl<HseClk> for Rcc<RccInt> {
    fn stabilize(&self, _clk: HseClk) -> FiberFuture<()> {
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

        // Wait for the clock to stabilize.
        hserdy
    }
}

impl<RccInt: IntToken> ClkCtrl<Pll> for Rcc<RccInt> {
    fn configure(&self, clk: Pll) {
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
    }
}

impl<RccInt: IntToken> StabilingClkCtrl<Pll> for Rcc<RccInt> {
    fn stabilize(&self, _clk: Pll) -> FiberFuture<()> {
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

        // Enable the clock.
        self.rcc.rcc_cr.modify(|r| r.set_pllon());

        // Wait for the clock to stabilize.
        pllrdy
    }
}

impl<RccInt: IntToken> ClkCtrl<PClk1> for Rcc<RccInt> {
    fn configure(&self, clk: PClk1) {
        self.rcc.rcc_cfgr.modify(|r| {
            let ppre1 = match clk.ppre1 {
                1 => 0b000,
                2 => 0b100,
                4 => 0b101,
                8 => 0b110,
                16 => 0b111,
                _ => unreachable!(),
            };
            r.write_ppre1(ppre1)
        });
    }
}

impl<RccInt: IntToken> ClkCtrl<PClk2> for Rcc<RccInt> {
    fn configure(&self, clk: PClk2) {
        self.rcc.rcc_cfgr.modify(|r| {
            let ppre2 = match clk.ppre2 {
                1 => 0b000,
                2 => 0b100,
                4 => 0b101,
                8 => 0b110,
                16 => 0b111,
                _ => unreachable!(),
            };
            r.write_ppre2(ppre2)
        });
    }
}

impl<RccInt: IntToken> MuxCtrl<PllSrcMuxSignal> for Rcc<RccInt> {
    fn select(&self, signal: PllSrcMuxSignal) {
        match signal {
            PllSrcMuxSignal::Hsi(_) => self.rcc.rcc_pllcfgr.modify(|r| r.clear_pllsrc()),
            PllSrcMuxSignal::Hse(_) => self.rcc.rcc_pllcfgr.modify(|r| r.set_pllsrc()),
        }
    }
}

impl<RccInt: IntToken> MuxCtrl<SysClkMuxSignal> for Rcc<RccInt> {
    fn select(&self, signal: SysClkMuxSignal) {
        let sw = match signal {
            SysClkMuxSignal::Hsi(_) => 0b00,
            SysClkMuxSignal::Hse(_) => 0b01,
            SysClkMuxSignal::Pll(_) => 0b10,
        };
        self.rcc.rcc_cfgr.modify(|r| r.write_sw(sw));
    }
}
