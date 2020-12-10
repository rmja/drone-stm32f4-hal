use crate::{diverged::UartDiverged, rx::UartRxDrv, tx::UartTxDrv};
use core::marker::PhantomData;
use drone_cortexm::{fib, reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::{
    dma::ch::{DmaChMap, DmaChPeriph},
    uart::{traits::*, UartMap, UartPeriph},
};
use drone_stm32f4_dma_drv::{DmaChCfg, DmaStCh4, DmaStChToken};
use drone_stm32f4_rcc_drv::{clktree::*, traits::ConfiguredClk};

pub mod config {
    use super::*;

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

    /// Create a new uart setup with sensible defaults.
    fn new_setup<Uart: UartMap, UartInt: IntToken, Clk: PClkToken>(
        uart: UartPeriph<Uart>,
        uart_int: UartInt,
        clk: ConfiguredClk<Clk>,
    ) -> UartSetup<Uart, UartInt, Clk> {
        UartSetup {
            uart,
            uart_int,
            clk,
            baud_rate: BaudRate::Nominal(9_600),
            data_bits: 8,
            parity: Parity::None,
            stop_bits: StopBits::One,
            oversampling: 16,
        }
    }

    macro_rules! uart_setup {
        ($name:ident, $uart:ident, $pclk:ident) => {
            impl<UartInt: IntToken>
                UartSetup<drone_stm32_map::periph::uart::$uart, UartInt, $pclk>
            {
                /// Create a new 9600 8N1 uart setup with sensible defaults.
                pub fn $name(
                    uart: UartPeriph<drone_stm32_map::periph::uart::$uart>,
                    uart_int: UartInt,
                    clk: ConfiguredClk<$pclk>,
                ) -> UartSetup<drone_stm32_map::periph::uart::$uart, UartInt, $pclk> {
                    new_setup(uart, uart_int, clk)
                }
            }
        };
    }

    #[cfg(any(
        stm32_mcu = "stm32f401",
        stm32_mcu = "stm32f405",
        stm32_mcu = "stm32f407",
        stm32_mcu = "stm32f410",
        stm32_mcu = "stm32f411",
        stm32_mcu = "stm32f412",
        stm32_mcu = "stm32f413",
        stm32_mcu = "stm32f427",
        stm32_mcu = "stm32f429",
        stm32_mcu = "stm32f446",
        stm32_mcu = "stm32f469",
    ))]
    uart_setup!(usart1, Usart1, PClk2);

    #[cfg(any(
        stm32_mcu = "stm32f401",
        stm32_mcu = "stm32f405",
        stm32_mcu = "stm32f407",
        stm32_mcu = "stm32f410",
        stm32_mcu = "stm32f411",
        stm32_mcu = "stm32f412",
        stm32_mcu = "stm32f413",
        stm32_mcu = "stm32f427",
        stm32_mcu = "stm32f429",
        stm32_mcu = "stm32f446",
        stm32_mcu = "stm32f469",
    ))]
    uart_setup!(usart2, Usart2, PClk1);

    #[cfg(any(
        stm32_mcu = "stm32f405",
        stm32_mcu = "stm32f407",
        stm32_mcu = "stm32f412",
        stm32_mcu = "stm32f413",
        stm32_mcu = "stm32f417",
        stm32_mcu = "stm32f427",
        stm32_mcu = "stm32f429",
        stm32_mcu = "stm32f437",
        stm32_mcu = "stm32f446",
        stm32_mcu = "stm32f469",
    ))]
    uart_setup!(usart3, Usart3, PClk1);

    #[cfg(any(
        stm32_mcu = "stm32f405",
        stm32_mcu = "stm32f407",
        stm32_mcu = "stm32f413",
        stm32_mcu = "stm32f417",
        stm32_mcu = "stm32f427",
        stm32_mcu = "stm32f429",
        stm32_mcu = "stm32f437",
        stm32_mcu = "stm32f446",
        stm32_mcu = "stm32f469",
    ))]
    uart_setup!(uart4, Uart4, PClk1);

    #[cfg(any(
        stm32_mcu = "stm32f405",
        stm32_mcu = "stm32f407",
        stm32_mcu = "stm32f405",
        stm32_mcu = "stm32f413",
        stm32_mcu = "stm32f417",
        stm32_mcu = "stm32f427",
        stm32_mcu = "stm32f429",
        stm32_mcu = "stm32f437",
        stm32_mcu = "stm32f446",
        stm32_mcu = "stm32f469",
    ))]
    uart_setup!(uart5, Uart5, PClk1);

    #[cfg(any(
        stm32_mcu = "stm32f401",
        stm32_mcu = "stm32f405",
        stm32_mcu = "stm32f407",
        stm32_mcu = "stm32f410",
        stm32_mcu = "stm32f411",
        stm32_mcu = "stm32f412",
        stm32_mcu = "stm32f413",
        stm32_mcu = "stm32f427",
        stm32_mcu = "stm32f429",
        stm32_mcu = "stm32f446",
        stm32_mcu = "stm32f469",
    ))]
    uart_setup!(usart6, Usart6, PClk2);

    #[cfg(any(
        stm32_mcu = "stm32f405",
        stm32_mcu = "stm32f407",
        stm32_mcu = "stm32f417",
        stm32_mcu = "stm32f427",
        stm32_mcu = "stm32f437",
        stm32_mcu = "stm32f469",
    ))]
    uart_setup!(uart7, Uart7, PClk1);

    #[cfg(any(
        stm32_mcu = "stm32f405",
        stm32_mcu = "stm32f407",
        stm32_mcu = "stm32f413",
        stm32_mcu = "stm32f417",
        stm32_mcu = "stm32f427",
        stm32_mcu = "stm32f437",
        stm32_mcu = "stm32f469",
    ))]
    uart_setup!(uart8, Uart8, PClk1);

    #[cfg(any(stm32_mcu = "stm32f413",))]
    uart_setup!(uart9, Uart9, PClk2);

    #[cfg(any(stm32_mcu = "stm32f413",))]
    uart_setup!(uart10, Uart10, PClk2);

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

/// Uart driver.
pub struct UartDrv<Uart: UartMap, UartInt: IntToken, Clk: PClkToken> {
    uart: UartDiverged<Uart>,
    uart_int: UartInt,
    clk: PhantomData<Clk>,
}

impl<Uart: UartMap, UartInt: IntToken, Clk: PClkToken> UartDrv<Uart, UartInt, Clk> {
    /// Sets up a new [`UartDrv`] from `setup` values.
    #[must_use]
    pub fn init(setup: config::UartSetup<Uart, UartInt, Clk>) -> Self {
        let config::UartSetup {
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

    /// Obtain a configured [`UartRxDrv`] from dma `setup` values.
    pub(crate) fn init_rx<DmaCh: DmaChMap, DmaStCh: DmaStChToken, DmaInt: IntToken>(
        &self,
        dma_cfg: DmaChCfg<DmaCh, DmaStCh, DmaInt>,
    ) -> UartRxDrv<Uart, UartInt, DmaCh, DmaInt> {
        let DmaChCfg {
            dma_ch,
            dma_int,
            dma_pl,
            ..
        } = dma_cfg;
        let mut rx = UartRxDrv {
            uart: &self.uart,
            uart_int: &self.uart_int,
            dma: dma_ch.into(),
            dma_int,
        };
        rx.init_dma_rx(DmaStCh::num(), dma_pl);
        rx
    }

    /// Obtain a configured [`UartTxDrv`] from dma `setup` values.
    pub(crate) fn init_tx<DmaCh: DmaChMap, DmaStCh: DmaStChToken, DmaInt: IntToken>(
        &self,
        dma_cfg: DmaChCfg<DmaCh, DmaStCh, DmaInt>,
    ) -> UartTxDrv<Uart, UartInt, DmaCh, DmaInt> {
        let DmaChCfg {
            dma_ch,
            dma_int,
            dma_pl,
            ..
        } = dma_cfg;
        let mut tx = UartTxDrv {
            uart: &self.uart,
            uart_int: &self.uart_int,
            dma: dma_ch.into(),
            dma_int,
        };
        tx.init_dma_tx(DmaStCh::num(), dma_pl);
        tx
    }

    fn init_uart(
        &mut self,
        clk: ConfiguredClk<Clk>,
        baud_rate: config::BaudRate,
        data_bits: u32,
        parity: config::Parity,
        stop_bits: config::StopBits,
        oversampling: u32,
    ) {
        use self::config::*;

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
            // Enable parity error interrupt
            r.peie().set(v);
            // Enable ORE or RXNE interrupt
            r.rxneie().set(v);
            // Enable uart after being fully configured.
            r.ue().set(v);
        });

        // Attach uart error handler
        let sr = self.uart.uart_sr;
        self.uart_int.add_fn(move || {
            let val = sr.load_val();
            handle_uart_err::<Uart>(&val, sr);
            fib::Yielded::<(), !>(())
        });
    }
}

// TODO: Do this with macros

impl<UartInt: IntToken, Clk: PClkToken>
    UartDrv<drone_stm32_map::periph::uart::Usart2, UartInt, Clk>
{
    pub fn init_usart1_rx<DmaInt: IntToken>(
        &self,
        dma_cfg: DmaChCfg<drone_stm32_map::periph::dma::ch::Dma1Ch5, DmaStCh4, DmaInt>,
    ) -> UartRxDrv<
        drone_stm32_map::periph::uart::Usart2,
        UartInt,
        drone_stm32_map::periph::dma::ch::Dma1Ch5,
        DmaInt,
    > {
        self.init_rx(dma_cfg)
    }
}

impl<UartInt: IntToken, Clk: PClkToken>
    UartDrv<drone_stm32_map::periph::uart::Usart2, UartInt, Clk>
{
    pub fn init_usart1_tx<DmaInt: IntToken>(
        &self,
        dma_cfg: DmaChCfg<drone_stm32_map::periph::dma::ch::Dma1Ch6, DmaStCh4, DmaInt>,
    ) -> UartTxDrv<
        drone_stm32_map::periph::uart::Usart2,
        UartInt,
        drone_stm32_map::periph::dma::ch::Dma1Ch6,
        DmaInt,
    > {
        self.init_tx(dma_cfg)
    }
}

fn uart_brr<Clk: PClkToken>(
    clk: ConfiguredClk<Clk>,
    baud_rate: config::BaudRate,
    oversampling: u32,
) -> (u32, u32) {
    match baud_rate {
        config::BaudRate::Nominal(baud_rate) => {
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
        config::BaudRate::Raw { div_man, div_frac } => (div_man, div_frac),
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

pub(crate) fn handle_dma_err<T: DmaChMap>(
    val: &T::DmaIsrVal,
    dma_isr_dmeif: T::CDmaIsrDmeif,
    dma_isr_feif: T::CDmaIsrFeif,
    dma_isr_teif: T::CDmaIsrTeif,
) {
    if dma_isr_teif.read(&val) {
        panic!("Transfer error");
    }
    if dma_isr_dmeif.read(&val) {
        panic!("Direct mode error");
    }
    if dma_isr_feif.read(&val) {
        panic!("FIFO error");
    }
}
