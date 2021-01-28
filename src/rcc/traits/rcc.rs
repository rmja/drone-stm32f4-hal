use core::ops::Deref;

use drone_core::fib::FiberFuture;
use drone_cortexm::thr::IntToken;

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

pub trait ClkCtrl<Clk> {
    /// Configures the clock `clk`.
    fn configure(&self, clk: Clk) -> ConfiguredClk<Clk>;
}

pub trait StabilizingClkCtrl<Clk> {
    /// Configures the clock `clk` and completes the future when the clock has stabilized.
    fn stabilize(&self, clk: Clk) -> FiberFuture<ConfiguredClk<Clk>>;
}

pub trait MuxCtrl<'a, RccInt: IntToken, MuxSignal, Clk> {
    type Builder;

    /// Select the source clock signal of a mux.
    fn select(
        &'a self,
        signal: MuxSignal,
        clk: ConfiguredClk<Clk>,
    ) -> Self::Builder;
}
