use self::traits::*;
use crate::{clktree::*, diverged::RccDiverged, periph::RccPeriph};
use core::marker::PhantomData;
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

impl<RccInt: IntToken> RccSetup<RccInt> {
    pub fn new(rcc: RccPeriph, rcc_int: RccInt) -> Self {
        Self {
            rcc,
            rcc_int,
        }
    }
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
    use core::ops::Deref;

    use super::*;

    #[derive(Copy, Clone)]
    pub struct ConfiguredClk<Clk> {
        pub(crate) clk: Clk,
    }

    impl<Clk> Deref for ConfiguredClk<Clk> {
        type Target = Clk;

        fn deref(&self) -> &Self::Target {
            &self.clk
        }
    }

    impl ConfiguredClk<Pll> {
        pub fn p(self) -> ConfiguredClk<PllClk<PllP>> {
            ConfiguredClk { clk: self.clk.p }
        }

        pub fn q(self) -> ConfiguredClk<PllClk<PllQ>> {
            ConfiguredClk { clk: self.clk.q }
        }
    }

    pub trait ClkCtrl<Clk> {
        /// Configures the clock `clk`.
        fn configure(&self, clk: Clk) -> ConfiguredClk<Clk>;
    }

    pub trait StabilizingClkCtrl<Clk> {
        /// Configures the clock `clk` and completes the future when the clock has stabilized.
        fn stabilize(&self, clk: Clk) -> FiberFuture<ConfiguredClk<Clk>>;
    }

    pub struct SelectedClkBuilder<'a, RccInt: IntToken, Clk> {
        pub(crate) rcc: &'a Rcc<RccInt>,
        pub(crate) _clk: PhantomData<Clk>,
    }

    pub trait MuxCtrl<RccInt: IntToken, MuxSignal, Clk> {
        /// Select the source clock signal of a mux.
        fn select(
            &self,
            signal: MuxSignal,
            clk: ConfiguredClk<Clk>,
        ) -> SelectedClkBuilder<RccInt, Clk>;
    }
}

impl<RccInt: IntToken> StabilizingClkCtrl<HseClk> for Rcc<RccInt> {
    fn stabilize(&self, clk: HseClk) -> FiberFuture<ConfiguredClk<HseClk>> {
        // Enable ready interrupt.
        self.rcc.rcc_cir.modify(|r| r.set_hserdyie());

        let reg::rcc::Cir {
            hserdyc, hserdyf, ..
        } = self.rcc.rcc_cir;

        // Attach a listener that will notify us when the clock has stabilized.
        let hserdy = self.rcc_int.add_future(fib::new_fn(move || {
            if hserdyf.read_bit() {
                hserdyc.set_bit();
                fib::Complete(ConfiguredClk { clk })
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

impl<RccInt: IntToken, SrcClk> StabilizingClkCtrl<Pll> for SelectedClkBuilder<'_, RccInt, SrcClk> {
    fn stabilize(&self, clk: Pll) -> FiberFuture<ConfiguredClk<Pll>> {
        let rcc = self.rcc;

        // Enable ready interrupt.
        rcc.rcc.rcc_cir.modify(|r| r.set_pllrdyie());

        let reg::rcc::Cir {
            pllrdyc, pllrdyf, ..
        } = rcc.rcc.rcc_cir;

        // Attach a listener that will notify us when the clock has stabilized.
        let pllrdy = rcc.rcc_int.add_future(fib::new_fn(move || {
            if pllrdyf.read_bit() {
                pllrdyc.set_bit();
                fib::Complete(ConfiguredClk { clk })
            } else {
                fib::Yielded(())
            }
        }));

        // Configure the clock.
        rcc.rcc.rcc_pllcfgr.modify(|r| {
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
        rcc.rcc.rcc_cr.modify(|r| r.set_pllon());

        // Wait for the clock to stabilize.
        pllrdy
    }
}

impl<RccInt: IntToken> ClkCtrl<PClk1> for Rcc<RccInt> {
    fn configure(&self, clk: PClk1) -> ConfiguredClk<PClk1> {
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
        ConfiguredClk { clk }
    }
}

impl<RccInt: IntToken> ClkCtrl<PClk2> for Rcc<RccInt> {
    fn configure(&self, clk: PClk2) -> ConfiguredClk<PClk2> {
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
        ConfiguredClk { clk }
    }
}

impl<RccInt: IntToken> MuxCtrl<RccInt, PllSrcMuxSignal, HsiClk> for Rcc<RccInt> {
    fn select(
        &self,
        signal: PllSrcMuxSignal,
        _clk: ConfiguredClk<HsiClk>,
    ) -> SelectedClkBuilder<RccInt, HsiClk> {
        assert!(matches!(signal, PllSrcMuxSignal::Hsi { .. }));
        self.rcc.rcc_pllcfgr.modify(|r| r.clear_pllsrc());
        SelectedClkBuilder {
            rcc: self,
            _clk: PhantomData,
        }
    }
}

impl<RccInt: IntToken> MuxCtrl<RccInt, PllSrcMuxSignal, HseClk> for Rcc<RccInt> {
    fn select(
        &self,
        signal: PllSrcMuxSignal,
        _clk: ConfiguredClk<HseClk>,
    ) -> SelectedClkBuilder<RccInt, HseClk> {
        assert!(matches!(signal, PllSrcMuxSignal::Hse { .. }));
        self.rcc.rcc_pllcfgr.modify(|r| r.set_pllsrc());
        SelectedClkBuilder {
            rcc: self,
            _clk: PhantomData,
        }
    }
}

impl<RccInt: IntToken> MuxCtrl<RccInt, SysClkMuxSignal, HsiClk> for Rcc<RccInt> {
    fn select(
        &self,
        signal: SysClkMuxSignal,
        _clk: ConfiguredClk<HsiClk>,
    ) -> SelectedClkBuilder<RccInt, HsiClk> {
        assert!(matches!(signal, SysClkMuxSignal::Hsi { .. }));
        self.rcc.rcc_cfgr.modify(|r| r.write_sw(0b00));
        SelectedClkBuilder {
            rcc: self,
            _clk: PhantomData,
        }
    }
}

impl<RccInt: IntToken> MuxCtrl<RccInt, SysClkMuxSignal, HseClk> for Rcc<RccInt> {
    fn select(
        &self,
        signal: SysClkMuxSignal,
        _clk: ConfiguredClk<HseClk>,
    ) -> SelectedClkBuilder<RccInt, HseClk> {
        assert!(matches!(signal, SysClkMuxSignal::Hse { .. }));
        self.rcc.rcc_cfgr.modify(|r| r.write_sw(0b01));
        SelectedClkBuilder {
            rcc: self,
            _clk: PhantomData,
        }
    }
}

impl<RccInt: IntToken> MuxCtrl<RccInt, SysClkMuxSignal, PllClk<PllP>> for Rcc<RccInt> {
    fn select(
        &self,
        signal: SysClkMuxSignal,
        _clk: ConfiguredClk<PllClk<PllP>>,
    ) -> SelectedClkBuilder<RccInt, PllClk<PllP>> {
        assert!(matches!(signal, SysClkMuxSignal::Pll { .. }));
        self.rcc.rcc_cfgr.modify(|r| r.write_sw(0b10));
        SelectedClkBuilder {
            rcc: self,
            _clk: PhantomData,
        }
    }
}
