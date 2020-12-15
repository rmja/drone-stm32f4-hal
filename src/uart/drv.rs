use self::config::*;
use crate::{diverged::UartDiverged, pins::*, rx::UartRxDrv, tx::UartTxDrv};
use core::marker::PhantomData;
use drone_cortexm::{fib, reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::{
    dma::ch::DmaChMap,
    uart::{traits::*, UartMap, UartPeriph},
};
use drone_stm32f4_dma_drv::{DmaChCfg, DmaStChToken};
use drone_stm32f4_rcc_drv::{clktree::*, traits::ConfiguredClk};

pub mod config {
    use super::*;
    pub use crate::pins::*;

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
}

#[macro_export]
macro_rules! uart_setup_init {
    ($uart:ident, $pclk:ident) => {
        impl<UartInt: drone_cortexm::thr::IntToken>
            crate::drv::config::UartSetupInit<drone_stm32_map::periph::uart::$uart, UartInt, $pclk>
            for crate::drv::config::UartSetup<drone_stm32_map::periph::uart::$uart, UartInt, $pclk>
        {
            fn init(
                uart: drone_stm32_map::periph::uart::UartPeriph<drone_stm32_map::periph::uart::$uart>,
                uart_int: UartInt,
                clk: drone_stm32f4_rcc_drv::traits::ConfiguredClk<$pclk>,
            ) -> crate::drv::config::UartSetup<drone_stm32_map::periph::uart::$uart, UartInt, $pclk> {
                Self {
                    uart,
                    uart_int,
                    clk,
                    baud_rate: crate::drv::config::BaudRate::Nominal(9_600),
                    data_bits: 8,
                    parity: crate::drv::config::Parity::None,
                    stop_bits: crate::drv::config::StopBits::One,
                    oversampling: 16,
                }
            }
        }
    };
}

/// Uart driver.
pub struct UartDrv<Uart: UartMap, UartInt: IntToken, Clk: PClkToken> {
    pub(crate) uart: UartDiverged<Uart>,
    pub(crate) uart_int: UartInt,
    clk: PhantomData<Clk>,
}

impl<Uart: UartMap, UartInt: IntToken, Clk: PClkToken> UartDrv<Uart, UartInt, Clk> {
    /// Sets up a new [`UartDrv`] from `setup` values.
    #[must_use]
    pub fn init(setup: UartSetup<Uart, UartInt, Clk>) -> Self {
        let UartSetup {
            uart,
            uart_int,
            clk,
            baud_rate,
            data_bits,
            stop_bits,
            parity,
            oversampling,
        } = setup;
        assert!(data_bits == 8 || data_bits == 9);
        assert!(oversampling == 8 || oversampling == 16);
        let mut drv = Self {
            uart: uart.into(),
            uart_int,
            clk: PhantomData,
        };
        drv.init_uart(clk, baud_rate, data_bits, parity, stop_bits, oversampling);
        drv
    }

    fn init_uart(
        &mut self,
        clk: ConfiguredClk<Clk>,
        baud_rate: BaudRate,
        data_bits: u32,
        parity: Parity,
        stop_bits: StopBits,
        oversampling: u32,
    ) {
        // Enable uart clock.
        self.uart.rcc_busenr_uarten.set_bit();

        // Configure uart.
        self.uart.uart_cr1.store_reg(|r, v| {
            // Do not enable uart before it is fully configured.

            // Word length.
            if data_bits == 9 {
                r.m().set(v);
            }

            // Parity.
            if parity != Parity::None {
                // Enable parity.
                r.pce().set(v);
                if parity == Parity::Odd {
                    // Parity selection: odd.
                    r.ps().set(v);
                }
            }

            // Oversampling.
            if oversampling == 8 {
                r.over8().set(v);
            }
        });
        self.uart.uart_cr2.store_reg(|r, v| {
            // Stop bits.
            r.stop().write(
                v,
                match stop_bits {
                    StopBits::One => 0,
                    StopBits::Half => 1,
                    StopBits::Two => 2,
                    StopBits::OneHalf => 3,
                },
            );
        });
        self.uart.uart_brr.store_reg(|r, v| {
            // Baud rate.
            let (div_man, div_frac) = uart_brr(clk, baud_rate, oversampling);
            r.div_mantissa().write(v, div_man);
            r.div_fraction().write(v, div_frac);
        });

        self.uart.uart_cr1.modify_reg(|r, v| {
            // Enable parity error interrupt.
            r.peie().set(v);
            // Enable ORE or RXNE interrupt.
            r.rxneie().set(v);
            // Enable uart after being fully configured.
            r.ue().set(v);
        });

        // Attach uart error handler.
        let sr = self.uart.uart_sr;
        self.uart_int.add_fn(move || {
            let val = sr.load_val();
            handle_uart_err::<Uart>(&val, sr);
            fib::Yielded::<(), !>(())
        });
    }
}

pub trait UartRxDrvInit<
    Uart: UartMap,
    UartInt: IntToken,
    DmaCh: DmaChMap,
    DmaStCh: DmaStChToken,
    Clk: PClkToken,
>
{
    /// Initialize a [`UartRxDrv`] from a configured dma channel.
    fn init_rx<DmaInt: IntToken, Tx>(
        &self,
        rx_cfg: DmaChCfg<DmaCh, DmaStCh, DmaInt>,
        rx_pins: &UartPins<Uart, Defined, Tx>,
    ) -> UartRxDrv<Uart, UartInt, DmaCh>;
}

pub trait UartTxDrvInit<
    Uart: UartMap,
    UartInt: IntToken,
    DmaCh: DmaChMap,
    DmaStCh: DmaStChToken,
    Clk: PClkToken,
>
{
    /// Initialize a [`UartTxDrv`] from a configured dma channel.
    fn init_tx<DmaInt: IntToken, Rx>(
        &self,
        tx_cfg: DmaChCfg<DmaCh, DmaStCh, DmaInt>,
        tx_pins: &UartPins<Uart, Rx, Defined>,
    ) -> UartTxDrv<Uart, UartInt, DmaCh, DmaInt>;
}

#[macro_export]
macro_rules! rx_drv_init {
    ($uart:ident, $ch:ident, $stch:ident) => {
        impl<UartInt: drone_cortexm::thr::IntToken, Clk: drone_stm32f4_rcc_drv::clktree::PClkToken>
            crate::drv::UartRxDrvInit<
                drone_stm32_map::periph::uart::$uart,
                UartInt,
                drone_stm32_map::periph::dma::ch::$ch,
                $stch,
                Clk,
            > for crate::drv::UartDrv<drone_stm32_map::periph::uart::$uart, UartInt, Clk>
        {
            fn init_rx<DmaRxInt: drone_cortexm::thr::IntToken, Tx>(
                &self,
                rx_cfg: drone_stm32f4_dma_drv::DmaChCfg<drone_stm32_map::periph::dma::ch::$ch, $stch, DmaRxInt>,
                _rx_pins: &crate::pins::UartPins<drone_stm32_map::periph::uart::$uart, Defined, Tx>,
            ) -> crate::rx::UartRxDrv<
                drone_stm32_map::periph::uart::$uart,
                UartInt,
                drone_stm32_map::periph::dma::ch::$ch,
            > {
                crate::rx::UartRxDrv::init(&self.uart, &self.uart_int, rx_cfg)
            }
        }
    };
}

#[macro_export]
macro_rules! tx_drv_init {
    ($uart:ident, $ch:ident, $stch:ident) => {
        impl<UartInt: drone_cortexm::thr::IntToken, Clk: drone_stm32f4_rcc_drv::clktree::PClkToken>
        crate::drv::UartTxDrvInit<
                drone_stm32_map::periph::uart::$uart,
                UartInt,
                drone_stm32_map::periph::dma::ch::$ch,
                $stch,
                Clk,
            > for crate::drv::UartDrv<drone_stm32_map::periph::uart::$uart, UartInt, Clk>
        {
            fn init_tx<DmaInt: drone_cortexm::thr::IntToken, Rx>(
                &self,
                tx_cfg: drone_stm32f4_dma_drv::DmaChCfg<drone_stm32_map::periph::dma::ch::$ch, $stch, DmaInt>,
                _tx_pins: &crate::pins::UartPins<drone_stm32_map::periph::uart::$uart, Rx, Defined>,
            ) -> crate::tx::UartTxDrv<
                drone_stm32_map::periph::uart::$uart,
                UartInt,
                drone_stm32_map::periph::dma::ch::$ch,
                DmaInt,
            > {
                crate::tx::UartTxDrv::init(&self.uart, &self.uart_int, tx_cfg)
            }
        }
    };
}

fn uart_brr<Clk: PClkToken>(
    clk: ConfiguredClk<Clk>,
    baud_rate: BaudRate,
    oversampling: u32,
) -> (u32, u32) {
    match baud_rate {
        BaudRate::Nominal(baud_rate) => {
            // Compute the uart divider for use by the baud rate register
            // according to eqn. 1 in PM0090 §30.3.4 page 978.
            // The computation of the divisor is as follows:
            //
            //                            f_pclk
            //       USARTDIV = ---------------------------
            //                  8 * (2 - over8) * baud_rate
            //                |
            //                V        25 * f_pclk
            // 100 * USARTDIV = ---------------------------
            //                  2 * (2 - over8) * baud_rate
            //
            // Note that 25 * f_pclk fits safely in a u32 as max f_pclk = 90_000_000.
            let f_pclk = clk.freq();
            let over8 = (oversampling == 8) as u32;
            let div100 = (25 * f_pclk) / (2 * (2 - over8) * baud_rate);
            let div_man = div100 / 100; // The mantissa part is: (100 * USARTDIV) / 100
            let rem100 = div100 - div_man * 100; // The reminder after the division: (100 * USARTDIV) % 100
            let div_frac = if over8 == 1 {
                // The frac field has 3 bits, 0..15
                (rem100 * 16 + 50) / 100
            } else {
                // The frac field has 4 bits, 0..31
                (rem100 * 32 + 50) / 100
            };

            (div_man, div_frac)
        }
        BaudRate::Raw { div_man, div_frac } => (div_man, div_frac),
    }
}

fn handle_uart_err<Uart: UartMap>(val: &Uart::UartSrVal, sr: Uart::CUartSr) {
    if sr.rxne().read(&val) {
        panic!("Read data register not empty");
    }
    if sr.ore().read(&val) {
        panic!("Overrun error");
    }
    if sr.nf().read(&val) {
        panic!("Noice");
    }
    if sr.fe().read(&val) {
        panic!("Framing error");
    }
    if sr.pe().read(&val) {
        panic!("Parity error");
    }
}