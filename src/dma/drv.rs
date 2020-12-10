use core::marker::PhantomData;
use drone_cortexm::{reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::dma::{
    ch::{DmaChMap, DmaChPeriph},
    DmaMap, DmaPeriph,
};

pub trait DmaStChToken {
    fn num() -> u32;
}

pub struct DmaStCh0;
pub struct DmaStCh1;
pub struct DmaStCh2;
pub struct DmaStCh3;
pub struct DmaStCh4;
pub struct DmaStCh5;
pub struct DmaStCh6;
pub struct DmaStCh7;

macro_rules! stch_num {
    ($stch:ident, $num:expr) => {
        impl DmaStChToken for $stch {
            fn num() -> u32 {
                $num
            }
        }
    };
}

stch_num!(DmaStCh0, 0);
stch_num!(DmaStCh1, 1);
stch_num!(DmaStCh2, 2);
stch_num!(DmaStCh3, 3);
stch_num!(DmaStCh4, 4);
stch_num!(DmaStCh5, 5);
stch_num!(DmaStCh6, 6);
stch_num!(DmaStCh7, 7);

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
    /// DMA channel peripheral (for F4 this is actually the stream).
    pub dma_ch: DmaChPeriph<DmaCh>,
    /// DMA channel interrupt (for F4 this is the stream interrupt).
    pub dma_int: DmaInt,
    /// DMA channel priority level.
    pub dma_pl: DmaPrio,
}

pub enum DmaPrio {
    Low,
    Medium,
    High,
    VeryHigh,
}

macro_rules! dma_setup {
    ($name:ident, $dma:ident, $ch:ident, $stch:ident) => {
        impl<DmaInt: IntToken>
            DmaChSetup<
                drone_stm32_map::periph::dma::$dma,
                drone_stm32_map::periph::dma::ch::$ch,
                $stch,
                DmaInt,
            >
        {
            pub fn $name(
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

dma_setup!(dma1_ch0_stch0, Dma1, Dma1Ch0, DmaStCh0);
dma_setup!(dma1_ch0_stch1, Dma1, Dma1Ch0, DmaStCh1);
dma_setup!(dma1_ch0_stch2, Dma1, Dma1Ch0, DmaStCh2);
dma_setup!(dma1_ch0_stch3, Dma1, Dma1Ch0, DmaStCh3);
dma_setup!(dma1_ch0_stch4, Dma1, Dma1Ch0, DmaStCh4);
dma_setup!(dma1_ch0_stch5, Dma1, Dma1Ch0, DmaStCh5);
dma_setup!(dma1_ch0_stch6, Dma1, Dma1Ch0, DmaStCh6);
dma_setup!(dma1_ch0_stch7, Dma1, Dma1Ch0, DmaStCh7);
dma_setup!(dma1_ch1_stch0, Dma1, Dma1Ch1, DmaStCh0);
dma_setup!(dma1_ch1_stch1, Dma1, Dma1Ch1, DmaStCh1);
dma_setup!(dma1_ch1_stch2, Dma1, Dma1Ch1, DmaStCh2);
dma_setup!(dma1_ch1_stch3, Dma1, Dma1Ch1, DmaStCh3);
dma_setup!(dma1_ch1_stch4, Dma1, Dma1Ch1, DmaStCh4);
dma_setup!(dma1_ch1_stch5, Dma1, Dma1Ch1, DmaStCh5);
dma_setup!(dma1_ch1_stch6, Dma1, Dma1Ch1, DmaStCh6);
dma_setup!(dma1_ch1_stch7, Dma1, Dma1Ch1, DmaStCh7);
dma_setup!(dma1_ch2_stch0, Dma1, Dma1Ch2, DmaStCh0);
dma_setup!(dma1_ch2_stch1, Dma1, Dma1Ch2, DmaStCh1);
dma_setup!(dma1_ch2_stch2, Dma1, Dma1Ch2, DmaStCh2);
dma_setup!(dma1_ch2_stch3, Dma1, Dma1Ch2, DmaStCh3);
dma_setup!(dma1_ch2_stch4, Dma1, Dma1Ch2, DmaStCh4);
dma_setup!(dma1_ch2_stch5, Dma1, Dma1Ch2, DmaStCh5);
dma_setup!(dma1_ch2_stch6, Dma1, Dma1Ch2, DmaStCh6);
dma_setup!(dma1_ch2_stch7, Dma1, Dma1Ch2, DmaStCh7);
dma_setup!(dma1_ch3_stch0, Dma1, Dma1Ch3, DmaStCh0);
dma_setup!(dma1_ch3_stch1, Dma1, Dma1Ch3, DmaStCh1);
dma_setup!(dma1_ch3_stch2, Dma1, Dma1Ch3, DmaStCh2);
dma_setup!(dma1_ch3_stch3, Dma1, Dma1Ch3, DmaStCh3);
dma_setup!(dma1_ch3_stch4, Dma1, Dma1Ch3, DmaStCh4);
dma_setup!(dma1_ch3_stch5, Dma1, Dma1Ch3, DmaStCh5);
dma_setup!(dma1_ch3_stch6, Dma1, Dma1Ch3, DmaStCh6);
dma_setup!(dma1_ch3_stch7, Dma1, Dma1Ch3, DmaStCh7);
dma_setup!(dma1_ch4_stch0, Dma1, Dma1Ch4, DmaStCh0);
dma_setup!(dma1_ch4_stch1, Dma1, Dma1Ch4, DmaStCh1);
dma_setup!(dma1_ch4_stch2, Dma1, Dma1Ch4, DmaStCh2);
dma_setup!(dma1_ch4_stch3, Dma1, Dma1Ch4, DmaStCh3);
dma_setup!(dma1_ch4_stch4, Dma1, Dma1Ch4, DmaStCh4);
dma_setup!(dma1_ch4_stch5, Dma1, Dma1Ch4, DmaStCh5);
dma_setup!(dma1_ch4_stch6, Dma1, Dma1Ch4, DmaStCh6);
dma_setup!(dma1_ch4_stch7, Dma1, Dma1Ch4, DmaStCh7);
dma_setup!(dma1_ch5_stch0, Dma1, Dma1Ch5, DmaStCh0);
dma_setup!(dma1_ch5_stch1, Dma1, Dma1Ch5, DmaStCh1);
dma_setup!(dma1_ch5_stch2, Dma1, Dma1Ch5, DmaStCh2);
dma_setup!(dma1_ch5_stch3, Dma1, Dma1Ch5, DmaStCh3);
dma_setup!(dma1_ch5_stch4, Dma1, Dma1Ch5, DmaStCh4);
dma_setup!(dma1_ch5_stch5, Dma1, Dma1Ch5, DmaStCh5);
dma_setup!(dma1_ch5_stch6, Dma1, Dma1Ch5, DmaStCh6);
dma_setup!(dma1_ch5_stch7, Dma1, Dma1Ch5, DmaStCh7);
dma_setup!(dma1_ch6_stch0, Dma1, Dma1Ch6, DmaStCh0);
dma_setup!(dma1_ch6_stch1, Dma1, Dma1Ch6, DmaStCh1);
dma_setup!(dma1_ch6_stch2, Dma1, Dma1Ch6, DmaStCh2);
dma_setup!(dma1_ch6_stch3, Dma1, Dma1Ch6, DmaStCh3);
dma_setup!(dma1_ch6_stch4, Dma1, Dma1Ch6, DmaStCh4);
dma_setup!(dma1_ch6_stch5, Dma1, Dma1Ch6, DmaStCh5);
dma_setup!(dma1_ch6_stch6, Dma1, Dma1Ch6, DmaStCh6);
dma_setup!(dma1_ch6_stch7, Dma1, Dma1Ch6, DmaStCh7);
dma_setup!(dma1_ch7_stch0, Dma1, Dma1Ch7, DmaStCh0);
dma_setup!(dma1_ch7_stch1, Dma1, Dma1Ch7, DmaStCh1);
dma_setup!(dma1_ch7_stch2, Dma1, Dma1Ch7, DmaStCh2);
dma_setup!(dma1_ch7_stch3, Dma1, Dma1Ch7, DmaStCh3);
dma_setup!(dma1_ch7_stch4, Dma1, Dma1Ch7, DmaStCh4);
dma_setup!(dma1_ch7_stch5, Dma1, Dma1Ch7, DmaStCh5);
dma_setup!(dma1_ch7_stch6, Dma1, Dma1Ch7, DmaStCh6);
dma_setup!(dma1_ch7_stch7, Dma1, Dma1Ch7, DmaStCh7);

dma_setup!(dma2_ch0_stch0, Dma2, Dma2Ch0, DmaStCh0);
dma_setup!(dma2_ch0_stch1, Dma2, Dma2Ch0, DmaStCh1);
dma_setup!(dma2_ch0_stch2, Dma2, Dma2Ch0, DmaStCh2);
dma_setup!(dma2_ch0_stch3, Dma2, Dma2Ch0, DmaStCh3);
dma_setup!(dma2_ch0_stch4, Dma2, Dma2Ch0, DmaStCh4);
dma_setup!(dma2_ch0_stch5, Dma2, Dma2Ch0, DmaStCh5);
dma_setup!(dma2_ch0_stch6, Dma2, Dma2Ch0, DmaStCh6);
dma_setup!(dma2_ch0_stch7, Dma2, Dma2Ch0, DmaStCh7);
dma_setup!(dma2_ch1_stch0, Dma2, Dma2Ch1, DmaStCh0);
dma_setup!(dma2_ch1_stch1, Dma2, Dma2Ch1, DmaStCh1);
dma_setup!(dma2_ch1_stch2, Dma2, Dma2Ch1, DmaStCh2);
dma_setup!(dma2_ch1_stch3, Dma2, Dma2Ch1, DmaStCh3);
dma_setup!(dma2_ch1_stch4, Dma2, Dma2Ch1, DmaStCh4);
dma_setup!(dma2_ch1_stch5, Dma2, Dma2Ch1, DmaStCh5);
dma_setup!(dma2_ch1_stch6, Dma2, Dma2Ch1, DmaStCh6);
dma_setup!(dma2_ch1_stch7, Dma2, Dma2Ch1, DmaStCh7);
dma_setup!(dma2_ch2_stch0, Dma2, Dma2Ch2, DmaStCh0);
dma_setup!(dma2_ch2_stch1, Dma2, Dma2Ch2, DmaStCh1);
dma_setup!(dma2_ch2_stch2, Dma2, Dma2Ch2, DmaStCh2);
dma_setup!(dma2_ch2_stch3, Dma2, Dma2Ch2, DmaStCh3);
dma_setup!(dma2_ch2_stch4, Dma2, Dma2Ch2, DmaStCh4);
dma_setup!(dma2_ch2_stch5, Dma2, Dma2Ch2, DmaStCh5);
dma_setup!(dma2_ch2_stch6, Dma2, Dma2Ch2, DmaStCh6);
dma_setup!(dma2_ch2_stch7, Dma2, Dma2Ch2, DmaStCh7);
dma_setup!(dma2_ch3_stch0, Dma2, Dma2Ch3, DmaStCh0);
dma_setup!(dma2_ch3_stch1, Dma2, Dma2Ch3, DmaStCh1);
dma_setup!(dma2_ch3_stch2, Dma2, Dma2Ch3, DmaStCh2);
dma_setup!(dma2_ch3_stch3, Dma2, Dma2Ch3, DmaStCh3);
dma_setup!(dma2_ch3_stch4, Dma2, Dma2Ch3, DmaStCh4);
dma_setup!(dma2_ch3_stch5, Dma2, Dma2Ch3, DmaStCh5);
dma_setup!(dma2_ch3_stch6, Dma2, Dma2Ch3, DmaStCh6);
dma_setup!(dma2_ch3_stch7, Dma2, Dma2Ch3, DmaStCh7);
dma_setup!(dma2_ch4_stch0, Dma2, Dma2Ch4, DmaStCh0);
dma_setup!(dma2_ch4_stch1, Dma2, Dma2Ch4, DmaStCh1);
dma_setup!(dma2_ch4_stch2, Dma2, Dma2Ch4, DmaStCh2);
dma_setup!(dma2_ch4_stch3, Dma2, Dma2Ch4, DmaStCh3);
dma_setup!(dma2_ch4_stch4, Dma2, Dma2Ch4, DmaStCh4);
dma_setup!(dma2_ch4_stch5, Dma2, Dma2Ch4, DmaStCh5);
dma_setup!(dma2_ch4_stch6, Dma2, Dma2Ch4, DmaStCh6);
dma_setup!(dma2_ch4_stch7, Dma2, Dma2Ch4, DmaStCh7);
dma_setup!(dma2_ch5_stch0, Dma2, Dma2Ch5, DmaStCh0);
dma_setup!(dma2_ch5_stch1, Dma2, Dma2Ch5, DmaStCh1);
dma_setup!(dma2_ch5_stch2, Dma2, Dma2Ch5, DmaStCh2);
dma_setup!(dma2_ch5_stch3, Dma2, Dma2Ch5, DmaStCh3);
dma_setup!(dma2_ch5_stch4, Dma2, Dma2Ch5, DmaStCh4);
dma_setup!(dma2_ch5_stch5, Dma2, Dma2Ch5, DmaStCh5);
dma_setup!(dma2_ch5_stch6, Dma2, Dma2Ch5, DmaStCh6);
dma_setup!(dma2_ch5_stch7, Dma2, Dma2Ch5, DmaStCh7);
dma_setup!(dma2_ch6_stch0, Dma2, Dma2Ch6, DmaStCh0);
dma_setup!(dma2_ch6_stch1, Dma2, Dma2Ch6, DmaStCh1);
dma_setup!(dma2_ch6_stch2, Dma2, Dma2Ch6, DmaStCh2);
dma_setup!(dma2_ch6_stch3, Dma2, Dma2Ch6, DmaStCh3);
dma_setup!(dma2_ch6_stch4, Dma2, Dma2Ch6, DmaStCh4);
dma_setup!(dma2_ch6_stch5, Dma2, Dma2Ch6, DmaStCh5);
dma_setup!(dma2_ch6_stch6, Dma2, Dma2Ch6, DmaStCh6);
dma_setup!(dma2_ch6_stch7, Dma2, Dma2Ch6, DmaStCh7);
dma_setup!(dma2_ch7_stch0, Dma2, Dma2Ch7, DmaStCh0);
dma_setup!(dma2_ch7_stch1, Dma2, Dma2Ch7, DmaStCh1);
dma_setup!(dma2_ch7_stch2, Dma2, Dma2Ch7, DmaStCh2);
dma_setup!(dma2_ch7_stch3, Dma2, Dma2Ch7, DmaStCh3);
dma_setup!(dma2_ch7_stch4, Dma2, Dma2Ch7, DmaStCh4);
dma_setup!(dma2_ch7_stch5, Dma2, Dma2Ch7, DmaStCh5);
dma_setup!(dma2_ch7_stch6, Dma2, Dma2Ch7, DmaStCh6);
dma_setup!(dma2_ch7_stch7, Dma2, Dma2Ch7, DmaStCh7);

pub struct DmaCfg<Dma: DmaMap> {
    dma: PhantomData<Dma>,
}

pub struct DmaChCfg<DmaCh: DmaChMap, DmaStCh, DmaInt: IntToken> {
    stch: PhantomData<DmaStCh>,
    pub dma_ch: DmaChPeriph<DmaCh>,
    pub dma_int: DmaInt,
    pub dma_pl: u32,
}

impl<Dma: DmaMap> DmaCfg<Dma> {
    /// Initialize a dma controller and enable its clock.
    pub fn init(dma: DmaPeriph<Dma>) -> DmaCfg<Dma> {
        dma.rcc_busenr_dmaen.set_bit();
        Self { dma: PhantomData }
    }

    /// Initialize a dma channel.
    pub fn init_ch<DmaCh: DmaChMap, StCh, DmaInt: IntToken>(
        &self,
        setup: DmaChSetup<Dma, DmaCh, StCh, DmaInt>,
    ) -> DmaChCfg<DmaCh, StCh, DmaInt> {
        let DmaChSetup {
            dma: PhantomData,
            stch: PhantomData,
            dma_ch,
            dma_int,
            dma_pl,
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
