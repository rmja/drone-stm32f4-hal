use self::config::*;
use crate::{diverged::SpiDiverged, master::SpiMasterDrv};
use core::marker::PhantomData;
use drone_cortexm::{fib, reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::{
    dma::ch::DmaChMap,
    spi::{traits::*, SpiCr1, SpiMap, SpiPeriph},
};
use drone_stm32f4_dma_drv::{DmaChCfg, DmaStChToken};
use drone_stm32f4_rcc_drv::{clktree::*, traits::ConfiguredClk};

pub mod config {
    use super::*;
    pub use crate::pins::*;

    pub struct SpiSetup<Spi: SpiMap + SpiCr1, SpiInt: IntToken, Clk: PClkToken> {
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

    pub trait NewSpiSetup<Spi: SpiMap + SpiCr1, SpiInt: IntToken, Clk: PClkToken> {
        /// Create a new spi setup with sensible defaults.
        fn new(
            spi: SpiPeriph<Spi>,
            spi_int: SpiInt,
            pins: SpiPins<Spi, Defined, Defined, Defined>,
            clk: ConfiguredClk<Clk>,
            baud_rate: BaudRate,
        ) -> SpiSetup<Spi, SpiInt, Clk>;
    }
    
    pub enum BaudRate {
        Max(u32),
        Custom(Prescaler),
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

#[macro_export]
macro_rules! spi_setup {
    ($spi:ident, $pclk:ident) => {
        impl<SpiInt: drone_cortexm::thr::IntToken> crate::drv::config::NewSpiSetup<drone_stm32_map::periph::spi::$spi, SpiInt, $pclk>
            for crate::drv::config::SpiSetup<drone_stm32_map::periph::spi::$spi, SpiInt, $pclk>
        {
            fn new(
                spi: drone_stm32_map::periph::spi::SpiPeriph<drone_stm32_map::periph::spi::$spi>,
                spi_int: SpiInt,
                _pins: crate::pins::SpiPins<drone_stm32_map::periph::spi::$spi, crate::pins::Defined, crate::pins::Defined, crate::pins::Defined>,
                clk: drone_stm32f4_rcc_drv::traits::ConfiguredClk<$pclk>,
                baud_rate: crate::drv::config::BaudRate,
            ) -> crate::drv::config::SpiSetup<drone_stm32_map::periph::spi::$spi, SpiInt, $pclk> {
                Self {
                    spi,
                    spi_int,
                    clk,
                    baud_rate,
                    clk_pol: crate::drv::config::ClkPol::Low,
                    first_bit: crate::drv::config::FirstBit::Msb,
                }
            }
        }
    };
}

pub struct SpiDrv<Spi: SpiMap + SpiCr1, SpiInt: IntToken, Clk: PClkToken> {
    pub(crate) spi: SpiDiverged<Spi>,
    spi_int: SpiInt,
    clk: PhantomData<Clk>,
}

impl<Spi: SpiMap + SpiCr1, SpiInt: IntToken, Clk: PClkToken> SpiDrv<Spi, SpiInt, Clk> {
    #[must_use]
    pub fn init(setup: config::SpiSetup<Spi, SpiInt, Clk>) -> SpiDrv<Spi, SpiInt, Clk> {
        let mut drv = Self {
            spi: setup.spi.into(),
            spi_int: setup.spi_int,
            clk: PhantomData,
        };
        drv.init_spi(setup.clk, setup.baud_rate, setup.clk_pol, setup.first_bit);
        drv
    }

    fn init_spi(
        &mut self,
        clk: ConfiguredClk<Clk>,
        baud_rate: BaudRate,
        clk_pol: ClkPol,
        first_bit: FirstBit,
    ) {
        use self::config::*;

        // Enable spi clock.
        self.spi.rcc_busenr_spien.set_bit();

        // Configure spi.
        self.spi.spi_cr1.store_reg(|r, v| {
            if first_bit == FirstBit::Lsb {
                r.lsbfirst().set(v);
            }

            // Baud rate control.
            r.br().write(v, spi_br(clk, baud_rate));

            // Clock polarity.
            if clk_pol == ClkPol::High {
                r.cpol().set(v);
            }

            // Clock phase.
            // TODO: Expose configuration option?
            r.cpha().clear(v);

            // Do not enable spi before it is fully configured.
        });

        // Attach spi error handler.
        let sr = self.spi.spi_sr;
        self.spi_int.add_fn(move || {
            let val = sr.load_val();
            handle_spi_err::<Spi>(&val, sr);
            fib::Yielded::<(), !>(())
        });

        // Enable error interrupt.
        self.spi.spi_cr2.store_reg(|r, v| {
            r.errie().set(v);
        });
    }
}

pub trait SpiDrvInit<
    Spi: SpiMap + SpiCr1,
    DmaRxCh: DmaChMap,
    DmaRxStCh: DmaStChToken,
    DmaTxCh: DmaChMap,
    DmaTxStCh: DmaStChToken,
>
{
    fn init_master<DmaRxInt: IntToken, DmaTxInt: IntToken>(
        &self,
        miso_cfg: DmaChCfg<DmaRxCh, DmaRxStCh, DmaRxInt>,
        mosi_cfg: DmaChCfg<DmaTxCh, DmaTxStCh, DmaTxInt>,
    ) -> SpiMasterDrv<Spi, DmaRxCh, DmaRxInt, DmaTxCh, DmaTxInt>;
}

#[macro_export]
macro_rules! master_drv_init {
    ($spi:ident, $miso_ch:ident, $miso_stch:ident, $mosi_ch:ident, $mosi_stch:ident) => {
        impl<SpiInt: drone_cortexm::thr::IntToken, Clk: drone_stm32f4_rcc_drv::clktree::PClkToken>
            crate::drv::SpiDrvInit<
                drone_stm32_map::periph::spi::$spi,
                drone_stm32_map::periph::dma::ch::$miso_ch,
                $miso_stch,
                drone_stm32_map::periph::dma::ch::$mosi_ch,
                $mosi_stch,
            > for crate::drv::SpiDrv<drone_stm32_map::periph::spi::$spi, SpiInt, Clk>
        {
            fn init_master<DmaRxInt: drone_cortexm::thr::IntToken, DmaTxInt: drone_cortexm::thr::IntToken>(
                &self,
                miso_cfg: drone_stm32f4_dma_drv::DmaChCfg<
                    drone_stm32_map::periph::dma::ch::$miso_ch,
                    $miso_stch,
                    DmaRxInt,
                >,
                mosi_cfg: drone_stm32f4_dma_drv::DmaChCfg<
                    drone_stm32_map::periph::dma::ch::$mosi_ch,
                    $mosi_stch,
                    DmaTxInt,
                >,
            ) -> crate::master::SpiMasterDrv<
                drone_stm32_map::periph::spi::$spi,
                drone_stm32_map::periph::dma::ch::$miso_ch,
                DmaRxInt,
                drone_stm32_map::periph::dma::ch::$mosi_ch,
                DmaTxInt,
            > {
                crate::master::SpiMasterDrv::init(&self.spi, miso_cfg, mosi_cfg)
            }
        }
    };
}

fn spi_br<Clk: PClkToken>(clk: ConfiguredClk<Clk>, baud_rate: BaudRate) -> u32 {
    use config::*;
    let f_pclk = clk.freq();
    let presc = match baud_rate {
        BaudRate::Max(baud_rate) => match f_pclk / baud_rate {
            0 => unreachable!(),
            1..=2 => Prescaler::Div2,
            3..=4 => Prescaler::Div4,
            5..=8 => Prescaler::Div8,
            9..=16 => Prescaler::Div16,
            17..=32 => Prescaler::Div32,
            33..=64 => Prescaler::Div64,
            65..=128 => Prescaler::Div128,
            _ => Prescaler::Div256,
        },
        BaudRate::Custom(prescaler) => prescaler,
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