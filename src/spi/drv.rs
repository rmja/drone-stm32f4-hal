use crate::{diverged::SpiDiverged, master::SpiMasterDrv};
use config::{BaudRate, ClkPol, FirstBit};
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
        /// The baud rate.
        pub baud_rate: BaudRate,
        /// The clock polarity.
        pub clk_pol: ClkPol,
        /// The bit transmission order.
        pub first_bit: FirstBit,
    }

    impl<Spi: SpiMap, SpiInt: IntToken> SpiSetup<Spi, SpiInt> {
        /// Create a new spi setup with sensible defaults.
        pub fn new(
            spi: SpiPeriph<Spi>,
            spi_int: SpiInt,
            baud_rate: BaudRate,
        ) -> SpiSetup<Spi, SpiInt> {
            SpiSetup {
                spi,
                spi_int,
                baud_rate,
                clk_pol: ClkPol::Low,
                first_bit: FirstBit::Msb,
            }
        }
    }

    pub enum BaudRate {
        Max { baud_rate: u32, f_pclk: u32 },
        Custom(Prescaler),
    }

    impl BaudRate {
        pub fn max(baud_rate: u32, f_pclk: u32) -> BaudRate {
            BaudRate::Max {
                baud_rate,
                f_pclk
            }
        }
        
        pub(crate) fn br(&self) -> u32 {
            let presc = match self {
                BaudRate::Max { baud_rate, f_pclk } => {
                    match f_pclk / baud_rate {
                        0 => unreachable!(),
                        1..=2 => Prescaler::Div2,
                        3..=4 => Prescaler::Div4,
                        5..=8 => Prescaler::Div8,
                        9..=16 => Prescaler::Div16,
                        17..=32 => Prescaler::Div32,
                        33..=64 => Prescaler::Div64,
                        65..=128 => Prescaler::Div128,
                        _ => Prescaler::Div256,
                    }
                },
                BaudRate::Custom(prescaler) => *prescaler,
            };

            match presc {
                Prescaler::Div2 => 0b000,
                Prescaler::Div4 => 0b001,
                Prescaler::Div8 => 0b010,
                Prescaler::Div16 => 0b011,
                Prescaler::Div32 => 0b100,
                Prescaler::Div64 => 0b101,
                Prescaler::Div128 => 0b110,
                Prescaler::Div256 => 0b111,
            }
        }
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
}

pub struct SpiDrv<Spi: SpiMap, SpiInt: IntToken> {
    spi: SpiDiverged<Spi>,
    spi_int: SpiInt,
}

impl<Spi: SpiMap, SpiInt: IntToken> SpiDrv<Spi, SpiInt> {
    #[must_use]
    pub fn init(setup: config::SpiSetup<Spi, SpiInt>) -> SpiDrv<Spi, SpiInt> {
        let config::SpiSetup {
            spi,
            spi_int,
            baud_rate,
            clk_pol,
            first_bit
        } = setup;
        let mut drv = SpiDrv {
            spi: spi.into(),
            spi_int,
        };
        drv.init_spi(baud_rate, clk_pol, first_bit);
        drv
    }

    pub fn master(&mut self) -> SpiMasterDrv {
        SpiMasterDrv {}
    }

    fn init_spi(&mut self, baud_rate: BaudRate, clk_pol: ClkPol, first_bit: FirstBit) {
        use self::config::*;

        // Enable spi clock.
        self.spi.rcc_busenr_spien.set_bit();

        // Configure spi.
        self.spi.spi_cr1.store_reg(|r, v| {
            // Do not enable spi before it is fully configured.

            // 8-bit data frame format.
            r.dff().clear(v);

            // Use software slave management, i.e. the app control slave selection.
            // TODO: Should the driver support hardware slave management?
            r.ssm().set(v);

            if first_bit == FirstBit::Msb {
                r.lsbfirst().clear(v);
            }
            else {
                r.lsbfirst().set(v);
            }

            // Baud rate control.
            r.br().write(baud_rate.br());

            // Master configuration.
            r.mstr().set(v);

            // Clock polarity.
            if clk_pol == ClkPol::Low {
                r.cpol().clear(v);
            }
            else {
                r.cpol().set(v);
            }

            // Clock phase.
            // TODO: Expose configuration option?
            r.cpha().clear(v);
        });

        self.spi.spi_cr2.modify_reg(|r, v| {
            // Enable error interrupt
            r.errie().set(v);
        });

        self.spi.spi_cr1.modify_reg(|r, v| {
            // Enable spi after being fully configured.
            r.spe().set(v);
        });

        // Attach spi error handler
        let sr = self.spi.spi_sr;
        self.spi_int.add_fn(move || {
            let val = sr.load_val();
            handle_spi_err::<Spi>(&val, sr);
            fib::Yielded::<(), !>(())
        });
    }
}

fn handle_spi_err<Spi: SpiMap>(val: &Spi::SpiSrVal, sr: Spi::CSpiSr) {
    if sr.fre().read(&val) {
        panic!("Frame format error");
    }
    if sr.ovr().read(&val) {
        panic!("Overrun error");
    }
    if sr.modf().read(&val) {
        panic!("Mode fault");
    }
    if sr.crcerr().read(&val) {
        panic!("CRC error");
    }
    if sr.udr().read(&val) {
        panic!("Underrun error");
    }
}