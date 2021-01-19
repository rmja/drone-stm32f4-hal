use self::traits::*;
use crate::{clktree::*, diverged::RccDiverged, periph::RccPeriph};
use core::cell::RefCell;
use core::marker::PhantomData;
use drone_core::bitfield::Bitfield;
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
        Self { rcc, rcc_int }
    }
}

#[derive(Clone, Copy, Bitfield)]
#[bitfield(hclk(rw, 0_u8), pclk1(rw, 1_u8), pclk2(rw, 2_u8))]
struct ConfiguredClocks(u8);

/// Rcc controller.
pub struct Rcc<RccInt: IntToken> {
    rcc: RccDiverged,
    rcc_int: RccInt,
    configured: RefCell<ConfiguredClocks>,
}

impl<RccInt: IntToken> Rcc<RccInt> {
    #[must_use]
    pub fn init(setup: RccSetup<RccInt>) -> Rcc<RccInt> {
        let RccSetup { rcc, rcc_int } = setup;
        Rcc {
            rcc: rcc.into(),
            rcc_int,
            configured: RefCell::new(ConfiguredClocks(0)),
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

    pub struct ConfiguredClkBuilder<'a, RccInt: IntToken, Clk> {
        pub(crate) rcc: &'a Rcc<RccInt>,
        pub(crate) clk: PhantomData<Clk>,
    }

    pub trait ClkCtrl<Clk> {
        /// Configures the clock `clk`.
        fn configure(&self, clk: Clk) -> ConfiguredClk<Clk>;
    }

    pub trait StabilizingClkCtrl<Clk> {
        /// Configures the clock `clk` and completes the future when the clock has stabilized.
        fn stabilize(&self, clk: Clk) -> FiberFuture<ConfiguredClk<Clk>>;
    }

    pub trait MuxCtrl<RccInt: IntToken, MuxSignal, Clk> {
        /// Select the source clock signal of a mux.
        fn select(
            &self,
            signal: MuxSignal,
            clk: ConfiguredClk<Clk>,
        ) -> ConfiguredClkBuilder<RccInt, Clk>;
    }
}

impl<RccInt: IntToken> StabilizingClkCtrl<HseClk> for Rcc<RccInt> {
    fn stabilize(&self, clk: HseClk) -> FiberFuture<ConfiguredClk<HseClk>> {
        assert!(self.rcc_int.is_int_enabled());

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

impl<RccInt: IntToken, SrcClk> StabilizingClkCtrl<Pll>
    for ConfiguredClkBuilder<'_, RccInt, SrcClk>
{
    fn stabilize(&self, clk: Pll) -> FiberFuture<ConfiguredClk<Pll>> {
        let rcc = self.rcc;
        assert!(rcc.rcc_int.is_int_enabled());

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

impl<RccInt: IntToken> ClkCtrl<HClk> for Rcc<RccInt> {
    fn configure(&self, clk: HClk) -> ConfiguredClk<HClk> {
        self.configured.borrow_mut().set_hclk();
        self.rcc.rcc_cfgr.modify(|r| {
            let hpre = match clk.hpre {
                1 => 0b0000,
                2 => 0b1000,
                4 => 0b1001,
                8 => 0b1010,
                16 => 0b1011,
                64 => 0b1100,
                128 => 0b1101,
                256 => 0b1110,
                512 => 0b1111,
                _ => unreachable!(),
            };
            r.write_hpre(hpre)
        });
        ConfiguredClk { clk }
    }
}

impl<RccInt: IntToken> ClkCtrl<PClk1> for Rcc<RccInt> {
    fn configure(&self, clk: PClk1) -> ConfiguredClk<PClk1> {
        self.configured.borrow_mut().set_pclk1();
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
        self.configured.borrow_mut().set_pclk2();
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
    ) -> ConfiguredClkBuilder<RccInt, HsiClk> {
        assert!(matches!(signal, PllSrcMuxSignal::Hsi { .. }));
        self.rcc.rcc_pllcfgr.modify(|r| r.clear_pllsrc());
        ConfiguredClkBuilder {
            rcc: self,
            clk: PhantomData,
        }
    }
}

impl<RccInt: IntToken> MuxCtrl<RccInt, PllSrcMuxSignal, HseClk> for Rcc<RccInt> {
    fn select(
        &self,
        signal: PllSrcMuxSignal,
        _clk: ConfiguredClk<HseClk>,
    ) -> ConfiguredClkBuilder<RccInt, HseClk> {
        assert!(matches!(signal, PllSrcMuxSignal::Hse { .. }));
        self.rcc.rcc_pllcfgr.modify(|r| r.set_pllsrc());
        ConfiguredClkBuilder {
            rcc: self,
            clk: PhantomData,
        }
    }
}

impl<RccInt: IntToken> MuxCtrl<RccInt, SysClkMuxSignal, HsiClk> for Rcc<RccInt> {
    fn select(
        &self,
        signal: SysClkMuxSignal,
        _clk: ConfiguredClk<HsiClk>,
    ) -> ConfiguredClkBuilder<RccInt, HsiClk> {
        assert!(matches!(signal, SysClkMuxSignal::Hsi { .. }));
        self.rcc.rcc_cfgr.modify(|r| r.write_sw(0b00));
        ConfiguredClkBuilder {
            rcc: self,
            clk: PhantomData,
        }
    }
}

impl<RccInt: IntToken> MuxCtrl<RccInt, SysClkMuxSignal, HseClk> for Rcc<RccInt> {
    fn select(
        &self,
        signal: SysClkMuxSignal,
        _clk: ConfiguredClk<HseClk>,
    ) -> ConfiguredClkBuilder<RccInt, HseClk> {
        assert!(matches!(signal, SysClkMuxSignal::Hse { .. }));
        self.rcc.rcc_cfgr.modify(|r| r.write_sw(0b01));
        ConfiguredClkBuilder {
            rcc: self,
            clk: PhantomData,
        }
    }
}

impl<RccInt: IntToken> MuxCtrl<RccInt, SysClkMuxSignal, PllClk<PllP>> for Rcc<RccInt> {
    fn select(
        &self,
        signal: SysClkMuxSignal,
        _clk: ConfiguredClk<PllClk<PllP>>,
    ) -> ConfiguredClkBuilder<RccInt, PllClk<PllP>> {
        assert!(matches!(signal, SysClkMuxSignal::Pll { .. }));
        // We need to make sure that HCLK, PCLK1, and PCLK2 are configured
        // to avoid overclocking of their max bus frequencies when setting the PLL as source.
        // Other sysclk signals are not fast enough to overclock the three buses.
        if self.configured.borrow().0 != 0b111 {
            panic!("Configure HCLK, PCLK1, and PCLK2 before selecting PLL as source.");
        }
        assert_eq!(0b111, self.configured.borrow().0);

        self.rcc.rcc_cfgr.modify(|r| r.write_sw(0b10));
        ConfiguredClkBuilder {
            rcc: self,
            clk: PhantomData,
        }
    }
}
