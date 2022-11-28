pub use crate::UartMap;
use drone_cortexm::thr::IntToken;
use drone_stm32_map::periph::uart::UartPeriph;
use drone_stm32f4_rcc_drv::{clktree::PClkToken, ConfiguredClk};

/// Uart setup.
pub struct UartSetup<Uart: UartMap, UartInt: IntToken, Clk: PClkToken> {
    /// Uart peripheral.
    pub uart: UartPeriph<Uart>,
    /// Uart global interrupt.
    pub uart_int: UartInt,
    /// Uart clock.
    pub clk: ConfiguredClk<Clk>,
    /// Baud rate.
    pub baud_rate: BaudRate,
    /// Data bits.
    pub data_bits: u32,
    /// Parity.
    pub parity: Parity,
    /// Stop bits.
    pub stop_bits: StopBits,
    /// Oversampling mode.
    pub oversampling: u32,
}

pub trait UartSetupInit<Uart: UartMap, UartInt: IntToken, Clk: PClkToken> {
    /// Create a new uart setup with the default configuration parameters 9600/8N1.
    fn init(
        uart: UartPeriph<Uart>,
        uart_int: UartInt,
        clk: ConfiguredClk<Clk>,
    ) -> UartSetup<Uart, UartInt, Clk>;
}

#[derive(Copy, Clone)]
pub enum BaudRate {
    Nominal(u32),
    Raw { div_man: u32, div_frac: u32 },
}

/// Uart parity.
#[derive(Copy, Clone, PartialEq)]
pub enum Parity {
    None,
    Even,
    Odd,
}

/// Uart stop bits.
#[derive(Copy, Clone, PartialEq)]
pub enum StopBits {
    #[doc = "½ stop bit."]
    Half,
    #[doc = "1 stop bit."]
    One,
    #[doc = "1½ stop bit."]
    OneHalf,
    #[doc = "2 stop bits."]
    Two,
}

#[macro_export]
macro_rules! uart_setup_init {
    ($uart:ident, $pclk:ident) => {
        impl<UartInt: drone_cortexm::thr::IntToken> crate::UartSetupInit<$uart, UartInt, $pclk>
            for crate::UartSetup<$uart, UartInt, $pclk>
        {
            fn init(
                uart: drone_stm32_map::periph::uart::UartPeriph<$uart>,
                uart_int: UartInt,
                clk: drone_stm32f4_rcc_drv::ConfiguredClk<$pclk>,
            ) -> crate::UartSetup<$uart, UartInt, $pclk> {
                Self {
                    uart,
                    uart_int,
                    clk,
                    baud_rate: crate::BaudRate::Nominal(9_600),
                    data_bits: 8,
                    parity: crate::Parity::None,
                    stop_bits: crate::StopBits::One,
                    oversampling: 16,
                }
            }
        }
    };
}
