use drone_cortexm::thr::IntToken;
use drone_stm32_map::periph::spi::SpiPeriph;
use drone_stm32f4_rcc_drv::{clktree::PClkToken, ConfiguredClk};

pub use crate::{pins::*, SpiMap};

pub struct SpiSetup<Spi: SpiMap, SpiInt: IntToken, Clk: PClkToken> {
    /// Spi peripheral.
    pub spi: SpiPeriph<Spi>,
    /// Spi global interrupt.
    pub spi_int: SpiInt,
    /// Spi clock.
    pub clk: ConfiguredClk<Clk>,
    /// The baud rate.
    pub baud_rate: BaudRate,
    /// The clock polarity.
    pub clk_pol: ClkPol,
    /// The bit transmission order.
    pub first_bit: FirstBit,
}

pub trait NewSpiSetup<Spi: SpiMap, SpiInt: IntToken, Clk: PClkToken> {
    /// Create a new spi setup with sensible defaults.
    fn new(
        spi: SpiPeriph<Spi>,
        spi_int: SpiInt,
        pins: SpiPins<Spi, Defined, Defined, Defined>,
        clk: ConfiguredClk<Clk>,
        baud_rate: BaudRate,
    ) -> Self;
}

pub enum BaudRate {
    Max(u32),
    Prescaler(Prescaler),
}

#[derive(Copy, Clone, PartialEq)]
pub enum Prescaler {
    Div2,
    Div4,
    Div8,
    Div16,
    Div32,
    Div64,
    Div128,
    Div256,
}

#[derive(Copy, Clone, PartialEq)]
pub enum ClkPol {
    Low,
    High,
}

#[derive(Copy, Clone, PartialEq)]
pub enum FirstBit {
    Msb,
    Lsb,
}

#[macro_export]
macro_rules! spi_setup {
    ($spi:ident, $pclk:ident) => {
        impl<SpiInt: drone_cortexm::thr::IntToken> crate::NewSpiSetup<$spi, SpiInt, $pclk>
            for crate::SpiSetup<$spi, SpiInt, $pclk>
        {
            fn new(
                spi: drone_stm32_map::periph::spi::SpiPeriph<$spi>,
                spi_int: SpiInt,
                _pins: crate::pins::SpiPins<
                    $spi,
                    crate::pins::Defined,
                    crate::pins::Defined,
                    crate::pins::Defined,
                >,
                clk: drone_stm32f4_rcc_drv::ConfiguredClk<$pclk>,
                baud_rate: crate::BaudRate,
            ) -> Self {
                Self {
                    spi,
                    spi_int,
                    clk,
                    baud_rate,
                    clk_pol: crate::ClkPol::Low,
                    first_bit: crate::FirstBit::Msb,
                }
            }
        }
    };
}
