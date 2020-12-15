use self::config::*;
use core::marker::PhantomData;
use drone_cortexm::{reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::dma::{
    ch::{DmaChMap, DmaChPeriph},
    DmaMap, DmaPeriph,
};

pub struct DmaStCh0;
pub struct DmaStCh1;
pub struct DmaStCh2;
pub struct DmaStCh3;
pub struct DmaStCh4;
pub struct DmaStCh5;
pub struct DmaStCh6;
pub struct DmaStCh7;

pub trait DmaStChToken {
    /// Get the stream channel number.
    fn num() -> u32;
}

macro_rules! stch_token {
    ($stch:ident, $num:expr) => {
        impl DmaStChToken for $stch {
            fn num() -> u32 {
                $num
            }
        }
    };
}

stch_token!(DmaStCh0, 0);
stch_token!(DmaStCh1, 1);
stch_token!(DmaStCh2, 2);
stch_token!(DmaStCh3, 3);
stch_token!(DmaStCh4, 4);
stch_token!(DmaStCh5, 5);
stch_token!(DmaStCh6, 6);
stch_token!(DmaStCh7, 7);

pub mod config {
    use super::*;

    /// Dma channel setup.
    /// Note that the nomenclature by ST has changed in between the STM32F4 and STM32L series.
    /// STM32F4 has a definition of streams which are divided into channels.
    /// STM32L has a definition of channels equivalent to F4 streams.
    /// Drone OS uses the naming from the STM32L series which is the newest generation,
    /// and this dma driver aligns with this naming.
    /// This is the reason why this type is named DmaChSetup and not DmaStSetup which would have been more correct for the F4 series.
    pub struct DmaChSetup<Dma: DmaMap, DmaCh: DmaChMap, DmaStCh, DmaInt: IntToken> {
        dma: PhantomData<Dma>,
        stch: PhantomData<DmaStCh>,
        /// Dma channel peripheral (for F4 this is actually the stream).
        pub dma_ch: DmaChPeriph<DmaCh>,
        /// Dma channel interrupt (for F4 this is the stream interrupt).
        pub dma_int: DmaInt,
        /// Dma channel priority level.
        pub dma_pl: DmaPrio,
    }

    pub enum DmaPrio {
        Low,
        Medium,
        High,
        VeryHigh,
    }

    pub trait NewDmaChSetup<Dma: DmaMap, DmaCh: DmaChMap, DmaStCh, DmaInt: IntToken> {
        /// Initialize a dma channel setup with medium priority level.
        fn new(ch: DmaChPeriph<DmaCh>, int: DmaInt) -> DmaChSetup<Dma, DmaCh, DmaStCh, DmaInt>;
    }

    macro_rules! dma_setup {
        ($dma:ident, $ch:ident, $stch:ident) => {
            impl<DmaInt: IntToken>
                NewDmaChSetup<
                    drone_stm32_map::periph::dma::$dma,
                    drone_stm32_map::periph::dma::ch::$ch,
                    $stch,
                    DmaInt,
                >
                for DmaChSetup<
                    drone_stm32_map::periph::dma::$dma,
                    drone_stm32_map::periph::dma::ch::$ch,
                    $stch,
                    DmaInt,
                >
            {
                fn new(
                    ch: DmaChPeriph<drone_stm32_map::periph::dma::ch::$ch>,
                    int: DmaInt,
                ) -> DmaChSetup<
                    drone_stm32_map::periph::dma::$dma,
                    drone_stm32_map::periph::dma::ch::$ch,
                    $stch,
                    DmaInt,
                > {
                    Self {
                        dma: PhantomData,
                        stch: PhantomData,
                        dma_ch: ch,
                        dma_int: int,
                        dma_pl: DmaPrio::Medium,
                    }
                }
            }
        };
    }

    dma_setup!(Dma1, Dma1Ch0, DmaStCh0);
    dma_setup!(Dma1, Dma1Ch0, DmaStCh1);
    dma_setup!(Dma1, Dma1Ch0, DmaStCh2);
    dma_setup!(Dma1, Dma1Ch0, DmaStCh3);
    dma_setup!(Dma1, Dma1Ch0, DmaStCh4);
    dma_setup!(Dma1, Dma1Ch0, DmaStCh5);
    dma_setup!(Dma1, Dma1Ch0, DmaStCh6);
    dma_setup!(Dma1, Dma1Ch0, DmaStCh7);
    dma_setup!(Dma1, Dma1Ch1, DmaStCh0);
    dma_setup!(Dma1, Dma1Ch1, DmaStCh1);
    dma_setup!(Dma1, Dma1Ch1, DmaStCh2);
    dma_setup!(Dma1, Dma1Ch1, DmaStCh3);
    dma_setup!(Dma1, Dma1Ch1, DmaStCh4);
    dma_setup!(Dma1, Dma1Ch1, DmaStCh5);
    dma_setup!(Dma1, Dma1Ch1, DmaStCh6);
    dma_setup!(Dma1, Dma1Ch1, DmaStCh7);
    dma_setup!(Dma1, Dma1Ch2, DmaStCh0);
    dma_setup!(Dma1, Dma1Ch2, DmaStCh1);
    dma_setup!(Dma1, Dma1Ch2, DmaStCh2);
    dma_setup!(Dma1, Dma1Ch2, DmaStCh3);
    dma_setup!(Dma1, Dma1Ch2, DmaStCh4);
    dma_setup!(Dma1, Dma1Ch2, DmaStCh5);
    dma_setup!(Dma1, Dma1Ch2, DmaStCh6);
    dma_setup!(Dma1, Dma1Ch2, DmaStCh7);
    dma_setup!(Dma1, Dma1Ch3, DmaStCh0);
    dma_setup!(Dma1, Dma1Ch3, DmaStCh1);
    dma_setup!(Dma1, Dma1Ch3, DmaStCh2);
    dma_setup!(Dma1, Dma1Ch3, DmaStCh3);
    dma_setup!(Dma1, Dma1Ch3, DmaStCh4);
    dma_setup!(Dma1, Dma1Ch3, DmaStCh5);
    dma_setup!(Dma1, Dma1Ch3, DmaStCh6);
    dma_setup!(Dma1, Dma1Ch3, DmaStCh7);
    dma_setup!(Dma1, Dma1Ch4, DmaStCh0);
    dma_setup!(Dma1, Dma1Ch4, DmaStCh1);
    dma_setup!(Dma1, Dma1Ch4, DmaStCh2);
    dma_setup!(Dma1, Dma1Ch4, DmaStCh3);
    dma_setup!(Dma1, Dma1Ch4, DmaStCh4);
    dma_setup!(Dma1, Dma1Ch4, DmaStCh5);
    dma_setup!(Dma1, Dma1Ch4, DmaStCh6);
    dma_setup!(Dma1, Dma1Ch4, DmaStCh7);
    dma_setup!(Dma1, Dma1Ch5, DmaStCh0);
    dma_setup!(Dma1, Dma1Ch5, DmaStCh1);
    dma_setup!(Dma1, Dma1Ch5, DmaStCh2);
    dma_setup!(Dma1, Dma1Ch5, DmaStCh3);
    dma_setup!(Dma1, Dma1Ch5, DmaStCh4);
    dma_setup!(Dma1, Dma1Ch5, DmaStCh5);
    dma_setup!(Dma1, Dma1Ch5, DmaStCh6);
    dma_setup!(Dma1, Dma1Ch5, DmaStCh7);
    dma_setup!(Dma1, Dma1Ch6, DmaStCh0);
    dma_setup!(Dma1, Dma1Ch6, DmaStCh1);
    dma_setup!(Dma1, Dma1Ch6, DmaStCh2);
    dma_setup!(Dma1, Dma1Ch6, DmaStCh3);
    dma_setup!(Dma1, Dma1Ch6, DmaStCh4);
    dma_setup!(Dma1, Dma1Ch6, DmaStCh5);
    dma_setup!(Dma1, Dma1Ch6, DmaStCh6);
    dma_setup!(Dma1, Dma1Ch6, DmaStCh7);
    dma_setup!(Dma1, Dma1Ch7, DmaStCh0);
    dma_setup!(Dma1, Dma1Ch7, DmaStCh1);
    dma_setup!(Dma1, Dma1Ch7, DmaStCh2);
    dma_setup!(Dma1, Dma1Ch7, DmaStCh3);
    dma_setup!(Dma1, Dma1Ch7, DmaStCh4);
    dma_setup!(Dma1, Dma1Ch7, DmaStCh5);
    dma_setup!(Dma1, Dma1Ch7, DmaStCh6);
    dma_setup!(Dma1, Dma1Ch7, DmaStCh7);

    dma_setup!(Dma2, Dma2Ch0, DmaStCh0);
    dma_setup!(Dma2, Dma2Ch0, DmaStCh1);
    dma_setup!(Dma2, Dma2Ch0, DmaStCh2);
    dma_setup!(Dma2, Dma2Ch0, DmaStCh3);
    dma_setup!(Dma2, Dma2Ch0, DmaStCh4);
    dma_setup!(Dma2, Dma2Ch0, DmaStCh5);
    dma_setup!(Dma2, Dma2Ch0, DmaStCh6);
    dma_setup!(Dma2, Dma2Ch0, DmaStCh7);
    dma_setup!(Dma2, Dma2Ch1, DmaStCh0);
    dma_setup!(Dma2, Dma2Ch1, DmaStCh1);
    dma_setup!(Dma2, Dma2Ch1, DmaStCh2);
    dma_setup!(Dma2, Dma2Ch1, DmaStCh3);
    dma_setup!(Dma2, Dma2Ch1, DmaStCh4);
    dma_setup!(Dma2, Dma2Ch1, DmaStCh5);
    dma_setup!(Dma2, Dma2Ch1, DmaStCh6);
    dma_setup!(Dma2, Dma2Ch1, DmaStCh7);
    dma_setup!(Dma2, Dma2Ch2, DmaStCh0);
    dma_setup!(Dma2, Dma2Ch2, DmaStCh1);
    dma_setup!(Dma2, Dma2Ch2, DmaStCh2);
    dma_setup!(Dma2, Dma2Ch2, DmaStCh3);
    dma_setup!(Dma2, Dma2Ch2, DmaStCh4);
    dma_setup!(Dma2, Dma2Ch2, DmaStCh5);
    dma_setup!(Dma2, Dma2Ch2, DmaStCh6);
    dma_setup!(Dma2, Dma2Ch2, DmaStCh7);
    dma_setup!(Dma2, Dma2Ch3, DmaStCh0);
    dma_setup!(Dma2, Dma2Ch3, DmaStCh1);
    dma_setup!(Dma2, Dma2Ch3, DmaStCh2);
    dma_setup!(Dma2, Dma2Ch3, DmaStCh3);
    dma_setup!(Dma2, Dma2Ch3, DmaStCh4);
    dma_setup!(Dma2, Dma2Ch3, DmaStCh5);
    dma_setup!(Dma2, Dma2Ch3, DmaStCh6);
    dma_setup!(Dma2, Dma2Ch3, DmaStCh7);
    dma_setup!(Dma2, Dma2Ch4, DmaStCh0);
    dma_setup!(Dma2, Dma2Ch4, DmaStCh1);
    dma_setup!(Dma2, Dma2Ch4, DmaStCh2);
    dma_setup!(Dma2, Dma2Ch4, DmaStCh3);
    dma_setup!(Dma2, Dma2Ch4, DmaStCh4);
    dma_setup!(Dma2, Dma2Ch4, DmaStCh5);
    dma_setup!(Dma2, Dma2Ch4, DmaStCh6);
    dma_setup!(Dma2, Dma2Ch4, DmaStCh7);
    dma_setup!(Dma2, Dma2Ch5, DmaStCh0);
    dma_setup!(Dma2, Dma2Ch5, DmaStCh1);
    dma_setup!(Dma2, Dma2Ch5, DmaStCh2);
    dma_setup!(Dma2, Dma2Ch5, DmaStCh3);
    dma_setup!(Dma2, Dma2Ch5, DmaStCh4);
    dma_setup!(Dma2, Dma2Ch5, DmaStCh5);
    dma_setup!(Dma2, Dma2Ch5, DmaStCh6);
    dma_setup!(Dma2, Dma2Ch5, DmaStCh7);
    dma_setup!(Dma2, Dma2Ch6, DmaStCh0);
    dma_setup!(Dma2, Dma2Ch6, DmaStCh1);
    dma_setup!(Dma2, Dma2Ch6, DmaStCh2);
    dma_setup!(Dma2, Dma2Ch6, DmaStCh3);
    dma_setup!(Dma2, Dma2Ch6, DmaStCh4);
    dma_setup!(Dma2, Dma2Ch6, DmaStCh5);
    dma_setup!(Dma2, Dma2Ch6, DmaStCh6);
    dma_setup!(Dma2, Dma2Ch6, DmaStCh7);
    dma_setup!(Dma2, Dma2Ch7, DmaStCh0);
    dma_setup!(Dma2, Dma2Ch7, DmaStCh1);
    dma_setup!(Dma2, Dma2Ch7, DmaStCh2);
    dma_setup!(Dma2, Dma2Ch7, DmaStCh3);
    dma_setup!(Dma2, Dma2Ch7, DmaStCh4);
    dma_setup!(Dma2, Dma2Ch7, DmaStCh5);
    dma_setup!(Dma2, Dma2Ch7, DmaStCh6);
    dma_setup!(Dma2, Dma2Ch7, DmaStCh7);
}

/// Dma controller configuration.
pub struct DmaCfg<Dma: DmaMap> {
    dma: PhantomData<Dma>,
}

impl<Dma: DmaMap> DmaCfg<Dma> {
    /// Initialize a dma controller and enable its clock.
    pub fn with_enabled_clock(dma: DmaPeriph<Dma>) -> DmaCfg<Dma> {
        dma.rcc_busenr_dmaen.set_bit();
        Self { dma: PhantomData }
    }

    /// Initialize a dma channel.
    pub fn ch<DmaCh: DmaChMap, StCh, DmaInt: IntToken>(
        &self,
        setup: DmaChSetup<Dma, DmaCh, StCh, DmaInt>,
    ) -> DmaChCfg<DmaCh, StCh, DmaInt> {
        let DmaChSetup {
            dma_ch,
            dma_int,
            dma_pl,
            ..
        } = setup;
        let pl = match dma_pl {
            DmaPrio::Low => 0b00,
            DmaPrio::Medium => 0b01,
            DmaPrio::High => 0b10,
            DmaPrio::VeryHigh => 0b11,
        };
        DmaChCfg {
            stch: PhantomData,
            dma_ch,
            dma_int,
            dma_pl: pl,
        }
    }
}

/// Dma channel configuration.
pub struct DmaChCfg<DmaCh: DmaChMap, DmaStCh, DmaInt: IntToken> {
    stch: PhantomData<DmaStCh>,
    /// Dma channel peripheral.
    pub dma_ch: DmaChPeriph<DmaCh>,
    /// Dma global interrupt.
    pub dma_int: DmaInt,
    /// Dma priority level.
    pub dma_pl: u32,
}