use crate::master::SpiMasterDrv;
use drone_cortexm::{fib, reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::{
    dma::ch::{DmaChMap, DmaChPeriph},
    spi::{traits::*, SpiMap, SpiPeriph},
};

pub mod config {
    use super::*;

    pub struct SpiSetup<Spi: SpiMap, SpiInt: IntToken> {
        /// Spi peripheral.
        pub spi: SpiPeriph<Spi>,
        /// Spi global interrupt.
        pub spi_int: SpiInt,
        /// The baud rate clock prescaler.
        /// baud_rate = f_pclk / baud_rate_prsc.
        pub baud_rate_prsc: Prsc,
        /// The clock polarity.
        pub clk_pol: ClkPol,
        /// The bit transmission order.
        pub first_bit: FirstBit,
    }

    impl<Spi: SpiMap, SpiInt: IntToken> SpiSetup<Spi, SpiInt> {
        /// Create a new spi setup with sensible defaults.
        pub fn default(spi: SpiPeriph<Spi>, spi_int: SpiInt) -> SpiSetup<Spi, SpiInt> {
            SpiSetup {
                spi,
                spi_int,
                baud_rate_prsc: Prsc::Prsc2,
                clk_pol: ClkPol::Low,
                first_bit: FirstBit::Msb,
            }
        }

        pub fn at(mut self, prsc: Prsc) -> Self {
            self.baud_rate_prsc = prsc;
            self
        }
    }

    pub enum Prsc {
        Prsc2,
        Prsc4,
        Prsc8,
        Prsc16,
        Prsc32,
        Prsc64,
        Prsc128,
        Prsc256,
    }

    pub enum ClkPol {
        Low,
        High,
    }

    pub enum FirstBit {
        Msb,
        Lsb,
    }
}

pub struct SpiDrv<Spi: SpiMap, SpiInt: IntToken> {
    spi: SpiPeriph<Spi>,
    spi_int: SpiInt,
}

impl<Spi: SpiMap, SpiInt: IntToken> SpiDrv<Spi, SpiInt> {
    #[must_use]
    pub fn init(setup: config::SpiSetup<Spi, SpiInt>) -> SpiDrv<Spi, SpiInt> {
        let config::SpiSetup { spi, spi_int, .. } = setup;
        let mut drv = SpiDrv { spi, spi_int };
        drv.init_spi();
        drv
    }

    pub fn master(&mut self) -> SpiMasterDrv {
        SpiMasterDrv {}
    }

    fn init_spi(&mut self) {
        // use self::config::*;

        // // Enable uart clock.
        // self.uart.rcc_busenr_uarten.set_bit();

        // // Configure uart.
        // self.uart.uart_cr1.store_reg(|r, v| {
        //     // Do not enable uart before it is fully configured.

        //     // Word length.
        //     if data_bits == DataBits::Nine {
        //         r.m().set(v);
        //     }

        //     // Parity.
        //     if parity != Parity::None {
        //         // Enable parity.
        //         r.pce().set(v);
        //         if parity == Parity::Odd {
        //             // Parity selection: odd.
        //             r.ps().set(v);
        //         }
        //     }

        //     // Oversampling.
        //     if oversampling == Oversampling::By8 {
        //         r.over8().set(v);
        //     }
        // });
        // self.uart.uart_cr2.store_reg(|r, v| {
        //     // Stop bits.
        //     r.stop().write(
        //         v,
        //         match stop_bits {
        //             StopBits::One => 0,
        //             StopBits::Half => 1,
        //             StopBits::Two => 2,
        //             StopBits::OneHalf => 3,
        //         },
        //     );
        // });
        // self.uart.uart_brr.store_reg(|r, v| {
        //     // Baud rate.
        //     let (div_man, div_frac) = clk.compute_brr(oversampling, baud_rate);
        //     r.div_mantissa().write(v, div_man);
        //     r.div_fraction().write(v, div_frac);
        // });

        // self.uart.uart_cr1.modify_reg(|r, v| {
        //     // Enable parity error interrupt
        //     r.peie().set(v);
        //     // Enable ORE or RXNE interrupt
        //     r.rxneie().set(v);
        //     // Enable uart after being fully configured.
        //     r.ue().set(v);
        // });

        // // Attach uart error handler
        // let sr = self.uart.uart_sr;
        // self.uart_int.add_fn(move || {
        //     let val = sr.load_val();
        //     handle_uart_err::<Uart>(&val, sr);
        //     fib::Yielded::<(), !>(())
        // });
    }
}
