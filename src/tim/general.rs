use drone_cortexm::reg::prelude::*;
use drone_stm32_map::periph::tim::{
    general::{GeneralTimMap, GeneralTimPeriph, traits::*},
};
use drone_stm32f4_rcc_drv::{clktree::*, traits::ConfiguredClk};

use crate::TimFreq;

pub struct GeneralTimSetup<Tim: GeneralTimMap, Clk: PClkToken> {
    pub tim: GeneralTimPeriph<Tim>,
    pub clk: ConfiguredClk<Clk>,
    pub freq: TimFreq,
}

pub trait NewGeneralTimSetup<Tim: GeneralTimMap, Clk: PClkToken> {
    /// Create a new tim setup with sensible defaults.
    fn new(
        tim: GeneralTimPeriph<Tim>,
        clk: ConfiguredClk<Clk>,
        freq: TimFreq,
    ) -> GeneralTimSetup<Tim, Clk>;
}

#[macro_export]
macro_rules! general_tim_setup {
    ($tim:ident, $pclk:ident) => {
        impl crate::general::NewGeneralTimSetup<$tim, $pclk>
            for crate::general::GeneralTimSetup<$tim, $pclk>
        {
            fn new(
                tim: drone_stm32_map::periph::tim::general::GeneralTimPeriph<$tim>,
                clk: drone_stm32f4_rcc_drv::traits::ConfiguredClk<$pclk>,
                freq: crate::TimFreq,
            ) -> Self
            {
                Self {
                    tim,
                    clk,
                    freq,
                }
            }
        }
    };
}

pub struct GeneralTimCfg<Tim: GeneralTimMap, Clk: PClkToken> {
    tim: GeneralTimPeriph<Tim>,
    clk: ConfiguredClk<Clk>,
}

impl<Tim: GeneralTimMap, Clk: PClkToken> GeneralTimCfg<Tim, Clk> {
    /// Initialize a general timer with the correct prescaler.
    #[must_use]
    pub fn with_enabled_clock(setup: GeneralTimSetup<Tim, Clk>) -> Self {
        let GeneralTimSetup {
            tim,
            clk,
            freq,
        } = setup;
        tim.rcc_busenr_timen.set_bit();

        // Set prescaler
        tim.tim_psc.psc().write_bits(tim_psc(&clk, freq) as u32);

        Self { tim, clk }
    }

    /// Disable the port clock.
    pub unsafe fn disable_clock(&self) {
        self.tim.rcc_busenr_timen.clear_bit();
    }

    /// Release the timer peripheral.
    pub fn release(self) -> GeneralTimPeriph<Tim> {
        self.tim
    }
}

fn tim_psc<Clk: PClkToken>(clk: &ConfiguredClk<Clk>, freq: TimFreq) -> u16 {
    let f_pclk_timer = clk.freq() * 2; // The PCLK is multipled by 2 before it enters the timer, see the clock tree for reference.
    match freq {
        TimFreq::Nominal(freq) => (((f_pclk_timer + (freq/2)) / freq) - 1) as u16,
        TimFreq::Prescaler(prescaler) => prescaler - 1,
    }
}