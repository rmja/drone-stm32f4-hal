use crate::{UartMap, setup::*, diverged::UartDiverged, pins::*, rx::UartRxDrv, tx::UartTxDrv};
use core::marker::PhantomData;
use alloc::sync::Arc;
use drone_cortexm::{fib, reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::{
    dma::ch::DmaChMap,
    uart::traits::*,
};
use drone_stm32f4_dma_drv::{DmaChCfg, DmaStChToken};
use drone_stm32f4_rcc_drv::{clktree::*, ConfiguredClk};

/// Uart driver.
pub struct UartDrv<Uart: UartMap, UartInt: IntToken, Clk: PClkToken> {
    pub(crate) uart: Arc<UartDiverged<Uart>>,
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
            uart: Arc::new(uart.into()),
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

pub trait IntoRxDrv<
    Uart: UartMap,
    UartInt: IntToken,
    DmaCh: DmaChMap,
    DmaStCh: DmaStChToken,
    Clk: PClkToken,
>
{
    /// Let the driver run in RX only for a configured dma channel.
    fn into_rx<DmaInt: IntToken, Tx>(
        self,
        rx_cfg: DmaChCfg<DmaCh, DmaStCh, DmaInt>,
        rx_pins: &UartPins<Uart, Defined, Tx>,
    ) -> UartRxDrv<Uart, UartInt, DmaCh>;
}

pub trait IntoTxDrv<
    Uart: UartMap,
    UartInt: IntToken,
    DmaCh: DmaChMap,
    DmaStCh: DmaStChToken,
    Clk: PClkToken,
>
{
    /// Let the driver run in TX only for a configured dma channel.
    fn into_tx<DmaInt: IntToken, Rx>(
        self,
        tx_cfg: DmaChCfg<DmaCh, DmaStCh, DmaInt>,
        tx_pins: &UartPins<Uart, Rx, Defined>,
    ) -> UartTxDrv<Uart, UartInt, DmaCh, DmaInt>;
}

pub trait IntoTrxDrv<
    Uart: UartMap,
    UartInt: IntToken,
    TxDmaCh: DmaChMap,
    TxDmaStCh: DmaStChToken,
    RxDmaCh: DmaChMap,
    RxDmaStCh: DmaStChToken,
    Clk: PClkToken,
>
{
    /// Let the driver run in TX and RX for configured dma channels.
    fn into_trx<TxDmaInt: IntToken, RxDmaInt: IntToken>(
        self,
        tx_cfg: DmaChCfg<TxDmaCh, TxDmaStCh, TxDmaInt>,
        rx_cfg: DmaChCfg<RxDmaCh, RxDmaStCh, RxDmaInt>,
        pins: &UartPins<Uart, Defined, Defined>,
    ) -> (
        UartTxDrv<Uart, UartInt, TxDmaCh, TxDmaInt>,
        UartRxDrv<Uart, UartInt, RxDmaCh>);
}

#[macro_export]
macro_rules! rx_drv_init {
    ($uart:ident; $ch:ident, $stch:ident) => {
        impl<
                UartInt: drone_cortexm::thr::IntToken,
                Clk: drone_stm32f4_rcc_drv::clktree::PClkToken,
            >
            crate::drv::IntoRxDrv<
                drone_stm32_map::periph::uart::$uart,
                UartInt,
                drone_stm32_map::periph::dma::ch::$ch,
                $stch,
                Clk,
            > for crate::drv::UartDrv<drone_stm32_map::periph::uart::$uart, UartInt, Clk>
        {
            fn into_rx<DmaRxInt: drone_cortexm::thr::IntToken, Tx>(
                self,
                rx_cfg: drone_stm32f4_dma_drv::DmaChCfg<
                    drone_stm32_map::periph::dma::ch::$ch,
                    $stch,
                    DmaRxInt,
                >,
                _rx_pins: &crate::pins::UartPins<drone_stm32_map::periph::uart::$uart, Defined, Tx>,
            ) -> crate::rx::UartRxDrv<
                drone_stm32_map::periph::uart::$uart,
                UartInt,
                drone_stm32_map::periph::dma::ch::$ch,
            > {
                crate::rx::UartRxDrv::init(self.uart, self.uart_int, rx_cfg)
            }
        }
    };
}

#[macro_export]
macro_rules! tx_drv_init {
    ($uart:ident; $ch:ident, $stch:ident) => {
        impl<
                UartInt: drone_cortexm::thr::IntToken,
                Clk: drone_stm32f4_rcc_drv::clktree::PClkToken,
            >
            crate::drv::IntoTxDrv<
                drone_stm32_map::periph::uart::$uart,
                UartInt,
                drone_stm32_map::periph::dma::ch::$ch,
                $stch,
                Clk,
            > for crate::drv::UartDrv<drone_stm32_map::periph::uart::$uart, UartInt, Clk>
        {
            fn into_tx<DmaInt: drone_cortexm::thr::IntToken, Rx>(
                self,
                tx_cfg: drone_stm32f4_dma_drv::DmaChCfg<
                    drone_stm32_map::periph::dma::ch::$ch,
                    $stch,
                    DmaInt,
                >,
                _tx_pins: &crate::pins::UartPins<drone_stm32_map::periph::uart::$uart, Rx, Defined>,
            ) -> crate::tx::UartTxDrv<
                drone_stm32_map::periph::uart::$uart,
                UartInt,
                drone_stm32_map::periph::dma::ch::$ch,
                DmaInt,
            > {
                crate::tx::UartTxDrv::init(self.uart, self.uart_int, tx_cfg)
            }
        }
    };
}

#[macro_export]
macro_rules! trx_drv_init {
    ($uart:ident; $tx_ch:ident, $tx_stch:ident; $rx_ch:ident, $rx_stch:ident) => {
        impl<
                UartInt: drone_cortexm::thr::IntToken,
                Clk: drone_stm32f4_rcc_drv::clktree::PClkToken,
            >
            crate::drv::IntoTrxDrv<
                drone_stm32_map::periph::uart::$uart,
                UartInt,
                drone_stm32_map::periph::dma::ch::$tx_ch,
                $tx_stch,
                drone_stm32_map::periph::dma::ch::$rx_ch,
                $rx_stch,
                Clk,
            > for crate::drv::UartDrv<drone_stm32_map::periph::uart::$uart, UartInt, Clk>
        {
            fn into_trx<TxDmaInt: drone_cortexm::thr::IntToken, RxDmaInt: drone_cortexm::thr::IntToken>(
                self,
                tx_cfg: drone_stm32f4_dma_drv::DmaChCfg<
                    drone_stm32_map::periph::dma::ch::$tx_ch,
                    $tx_stch,
                    TxDmaInt,
                >,
                rx_cfg: drone_stm32f4_dma_drv::DmaChCfg<
                    drone_stm32_map::periph::dma::ch::$rx_ch,
                    $rx_stch,
                    RxDmaInt,
                >,
                _pins: &crate::pins::UartPins<drone_stm32_map::periph::uart::$uart, Defined, Defined>,
            ) -> (crate::tx::UartTxDrv<
                drone_stm32_map::periph::uart::$uart,
                UartInt,
                drone_stm32_map::periph::dma::ch::$tx_ch,
                TxDmaInt,
            >, crate::rx::UartRxDrv<
                drone_stm32_map::periph::uart::$uart,
                UartInt,
                drone_stm32_map::periph::dma::ch::$rx_ch,
            >) {
                let tx = crate::tx::UartTxDrv::init(self.uart.clone(), self.uart_int, tx_cfg);
                let rx = crate::rx::UartRxDrv::init(self.uart, self.uart_int, rx_cfg);

                (tx, rx)
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
