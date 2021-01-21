use alloc::sync::Arc;
use core::marker::PhantomData;
use drone_cortexm::{reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::tim::general::{
    traits::*, GeneralTimMap, GeneralTimPeriph, TimCr1Cms, TimCr1Dir, TimCr2, TimSmcr,
};
use drone_stm32f4_rcc_drv::{clktree::*, traits::ConfiguredClk};

use crate::{
    shared::DontCare, traits::*, GeneralTimChDrv, GeneralTimCntDrv, GeneralTimOvfDrv, TimFreq,
};

pub struct GeneralTimSetup<Tim: GeneralTimMap, Int: IntToken, Clk: PClkToken> {
    /// The timer peripheral.
    pub tim: GeneralTimPeriph<Tim>,
    /// The timer interrupt.
    pub tim_int: Int,
    /// The configured timer clock.
    pub clk: ConfiguredClk<Clk>,
    /// The timer frequency.
    pub freq: TimFreq,
    /// The auto-reload value defining the top value for the timer. The default is 0xFFFF.
    pub arr: u32,
    /// Whether the timer should stop during debugging.
    pub debug_stop: bool,
}

pub trait NewGeneralTimSetup<Tim: GeneralTimMap, Int: IntToken, Clk: PClkToken> {
    /// Create a new tim setup with sensible defaults.
    fn new(
        tim: GeneralTimPeriph<Tim>,
        tim_int: Int,
        clk: ConfiguredClk<Clk>,
        freq: TimFreq,
    ) -> GeneralTimSetup<Tim, Int, Clk>;
}

#[macro_export]
macro_rules! general_tim_setup {
    ($tim:ident, $pclk:ident) => {
        impl<Int: drone_cortexm::thr::IntToken> crate::gen::NewGeneralTimSetup<$tim, Int, $pclk>
            for crate::gen::GeneralTimSetup<$tim, Int, $pclk>
        {
            fn new(
                tim: drone_stm32_map::periph::tim::general::GeneralTimPeriph<$tim>,
                tim_int: Int,
                clk: drone_stm32f4_rcc_drv::traits::ConfiguredClk<$pclk>,
                freq: crate::TimFreq,
            ) -> Self {
                Self {
                    tim,
                    tim_int,
                    clk,
                    freq,
                    arr: 0xFFFF,
                    debug_stop: false,
                }
            }
        }
    };
}

pub struct GeneralTimCfg<
    Tim: GeneralTimMap,
    Int: IntToken,
    Clk: PClkToken,
    Dir,
    Link,
    Ch1Mode,
    Ch2Mode,
    Ch3Mode,
    Ch4Mode,
> {
    pub(crate) tim: Arc<GeneralTimPeriph<Tim>>,
    pub(crate) tim_int: Int,
    pub(crate) clk: ConfiguredClk<Clk>,
    pub(crate) dir: PhantomData<Dir>,
    pub link: PhantomData<Link>,
    pub cnt: GeneralTimCntDrv<Tim>,
    pub ovf: GeneralTimOvfDrv<Tim, Int>,
    pub ch1: GeneralTimChDrv<Tim, Int, TimCh1, Ch1Mode>,
    pub ch2: GeneralTimChDrv<Tim, Int, TimCh2, Ch2Mode>,
    pub ch3: GeneralTimChDrv<Tim, Int, TimCh3, Ch3Mode>,
    pub ch4: GeneralTimChDrv<Tim, Int, TimCh4, Ch4Mode>,
}

impl<
        Tim: GeneralTimMap,
        Int: IntToken,
        Clk: PClkToken,
        Dir,
        Link,
        Ch1Mode,
        Ch2Mode,
        Ch3Mode,
        Ch4Mode,
    > GeneralTimCfg<Tim, Int, Clk, Dir, Link, Ch1Mode, Ch2Mode, Ch3Mode, Ch4Mode>
{
    pub(crate) fn into<ToDir, ToLink>(
        self,
    ) -> GeneralTimCfg<Tim, Int, Clk, ToDir, ToLink, Ch1Mode, Ch2Mode, Ch3Mode, Ch4Mode> {
        let Self {
            tim,
            tim_int,
            clk,
            cnt,
            ovf,
            ch1,
            ch2,
            ch3,
            ch4,
            ..
        } = self;
        GeneralTimCfg {
            tim,
            tim_int,
            clk,
            dir: PhantomData,
            link: PhantomData,
            cnt,
            ovf,
            ch1,
            ch2,
            ch3,
            ch4,
        }
    }

    pub fn reset_counter(&mut self) {
        self.tim.tim_cnt.cnt().write_bits(0);
    }
}

impl<Tim: GeneralTimMap, Int: IntToken, Clk: PClkToken>
    GeneralTimCfg<Tim, Int, Clk, DontCare, DefaultLink, DontCare, DontCare, DontCare, DontCare>
{
    /// Initialize a general timer with the correct prescaler.
    #[must_use]
    pub fn with_enabled_clock(setup: GeneralTimSetup<Tim, Int, Clk>) -> Self {
        let GeneralTimSetup {
            tim,
            tim_int,
            clk,
            freq,
            arr,
            debug_stop,
        } = setup;

        // Enable clock.
        tim.rcc_busenr_timen.set_bit();

        if debug_stop {
            tim.dbg_dbgmcu_timstop.set_bit();
        }

        // Set prescaler
        tim.tim_psc
            .psc()
            .write_bits(Self::tim_psc(&clk, freq) as u32);

        // Set some sensible register values.
        tim.tim_cr1.store_reg(|r, v| {
            // dir and cms are set for count direction is configured.
            r.udis().clear(v); // Enable counter overflow event generation
            r.urs().set(v); // Only counter overflow generates an update interrupt
            r.opm().clear(v); // Counter is not stopped at update event
            r.arpe().set(v) // Use buffered auto reload value
        });

        // Set the auto-reload register to a full period.
        // This defines the number of bits in the timer.
        tim.tim_arr.arr().write_bits(arr);

        // Re-initialize the counter and generate an update of the registers.
        tim.tim_egr.ug().set_bit();

        let tim = Arc::new(tim);
        Self {
            tim: tim.clone(),
            tim_int,
            clk,
            dir: PhantomData,
            link: PhantomData,
            cnt: GeneralTimCntDrv::new(tim.clone()),
            ovf: GeneralTimOvfDrv::new(tim.clone(), tim_int),
            ch1: GeneralTimChDrv::new(tim.clone(), tim_int),
            ch2: GeneralTimChDrv::new(tim.clone(), tim_int),
            ch3: GeneralTimChDrv::new(tim.clone(), tim_int),
            ch4: GeneralTimChDrv::new(tim.clone(), tim_int),
        }
    }

    fn tim_psc(clk: &ConfiguredClk<Clk>, freq: TimFreq) -> u16 {
        let f_pclk_timer = clk.freq() * 2; // The PCLK is multipled by 2 before it enters the timer, see the clock tree for reference.
        match freq {
            TimFreq::Nominal(freq) => (((f_pclk_timer + (freq / 2)) / freq) - 1) as u16,
            TimFreq::Prescaler(prescaler) => prescaler - 1,
        }
    }
}

impl<
        Tim: GeneralTimMap + TimCr1Dir + TimCr1Cms,
        Int: IntToken,
        Clk: PClkToken,
        Link,
        Ch1Mode,
        Ch2Mode,
        Ch3Mode,
        Ch4Mode,
    > GeneralTimCfg<Tim, Int, Clk, DontCare, Link, Ch1Mode, Ch2Mode, Ch3Mode, Ch4Mode>
{
    // Let the counter "count up".
    pub fn into_count_up(
        self,
    ) -> GeneralTimCfg<Tim, Int, Clk, DirCountUp, Link, Ch1Mode, Ch2Mode, Ch3Mode, Ch4Mode> {
        self.tim.tim_cr1.modify_reg(|r, v| {
            r.dir().clear(v); // Count up
            r.cms().write(v, 0b00); // Count up or down depending on the direction bit (i.e. count up)
        });
        self.into()
    }

    // Let the counter "count down".
    pub fn into_count_down(
        self,
    ) -> GeneralTimCfg<Tim, Int, Clk, DirCountDown, Link, Ch1Mode, Ch2Mode, Ch3Mode, Ch4Mode> {
        self.tim.tim_cr1.modify_reg(|r, v| {
            r.dir().set(v); // Count down
            r.cms().write(v, 0b00); // Count up or down depending on the direction bit (i.e. count down)
        });
        self.into()
    }
}

impl<
        Tim: GeneralTimMap + TimCr1Dir + TimCr1Cms + TimCr2 + TimSmcr,
        Int: IntToken,
        Clk: PClkToken,
        Dir,
        Ch1Mode,
        Ch2Mode,
        Ch3Mode,
        Ch4Mode,
    > GeneralTimCfg<Tim, Int, Clk, Dir, DefaultLink, Ch1Mode, Ch2Mode, Ch3Mode, Ch4Mode>
{
    // Let the timer be in master mode.
    pub fn into_master(
        self,
    ) -> GeneralTimCfg<Tim, Int, Clk, Dir, MasterLink<Tim>, Ch1Mode, Ch2Mode, Ch3Mode, Ch4Mode>
    {
        self.tim.tim_cr2.modify_reg(|r, v| {
            r.mms().write(v, 0b001) // Enable master mode selection
        });
        self.tim.tim_smcr.modify_reg(|r, v| {
            r.msm().set(v) // Enable master/slave mode
        });
        self.into()
    }
}

pub(crate) fn slave_of<Tim: GeneralTimMap + TimSmcr>(
    tim: &GeneralTimPeriph<Tim>,
    sms: u32,
    ts: u32,
) {
    tim.tim_smcr.store_reg(|r, v| {
        r.sms0_2().write(v, sms); // Slave mode selection
        r.ts().write(v, ts); // Trigger selection
    });
}

impl<
        Tim: GeneralTimMap,
        Int: IntToken,
        Clk: PClkToken,
        Dir,
        Link,
        Ch1Mode,
        Ch2Mode,
        Ch3Mode,
        Ch4Mode,
    > GeneralTimCfg<Tim, Int, Clk, Dir, Link, Ch1Mode, Ch2Mode, Ch3Mode, Ch4Mode>
{
    /// Disable the timer clock.
    pub unsafe fn disable_clock(&self) {
        self.tim.rcc_busenr_timen.clear_bit();
    }

    /// Start the timer counter.
    pub fn start(&self) {
        self.tim.tim_cr1.cen().set_bit();
    }

    /// Stop the timer counter.
    pub fn stop(&self) {
        self.tim.tim_cr1.cen().clear_bit();
    }

    /// Release the timer peripheral.
    pub fn release(self) -> GeneralTimPeriph<Tim> {
        let Self { tim, .. } = self;
        match Arc::try_unwrap(tim) {
            Ok(tim) => tim,
            Err(_) => unreachable!(),
        }
    }
}

pub trait ConfigureTimCh1<
    Tim: GeneralTimMap,
    Int: IntToken,
    Clk: PClkToken,
    Dir,
    Link,
    Ch2Mode,
    Ch3Mode,
    Ch4Mode,
>
{
    /// Configure the capture/compare channel 1.
    fn ch1<F, Ch1Mode>(
        self,
        configure: F,
    ) -> GeneralTimCfg<Tim, Int, Clk, Dir, Link, Ch1Mode, Ch2Mode, Ch3Mode, Ch4Mode>
    where
        F: FnOnce(
            GeneralTimChDrv<Tim, Int, TimCh1, DontCare>,
        ) -> GeneralTimChDrv<Tim, Int, TimCh1, Ch1Mode>;
}

pub trait ConfigureTimCh2<
    Tim: GeneralTimMap,
    Int: IntToken,
    Clk: PClkToken,
    Dir,
    Link,
    Ch1Mode,
    Ch3Mode,
    Ch4Mode,
>
{
    /// Configure the capture/compare channel 2.
    fn ch2<F, Ch2Mode>(
        self,
        configure: F,
    ) -> GeneralTimCfg<Tim, Int, Clk, Dir, Link, Ch1Mode, Ch2Mode, Ch3Mode, Ch4Mode>
    where
        F: FnOnce(
            GeneralTimChDrv<Tim, Int, TimCh2, DontCare>,
        ) -> GeneralTimChDrv<Tim, Int, TimCh2, Ch2Mode>;
}

pub trait ConfigureTimCh3<
    Tim: GeneralTimMap,
    Int: IntToken,
    Clk: PClkToken,
    Dir,
    Link,
    Ch1Mode,
    Ch2Mode,
    Ch4Mode,
>
{
    /// Configure the capture/compare channel 3.
    fn ch3<F, Ch3Mode>(
        self,
        configure: F,
    ) -> GeneralTimCfg<Tim, Int, Clk, Dir, Link, Ch1Mode, Ch2Mode, Ch3Mode, Ch4Mode>
    where
        F: FnOnce(
            GeneralTimChDrv<Tim, Int, TimCh3, DontCare>,
        ) -> GeneralTimChDrv<Tim, Int, TimCh3, Ch3Mode>;
}

pub trait ConfigureTimCh4<
    Tim: GeneralTimMap,
    Int: IntToken,
    Clk: PClkToken,
    Dir,
    Link,
    Ch1Mode,
    Ch2Mode,
    Ch3Mode,
>
{
    /// Configure the capture/compare channel 4.
    fn ch4<F, Ch4Mode>(
        self,
        configure: F,
    ) -> GeneralTimCfg<Tim, Int, Clk, Dir, Link, Ch1Mode, Ch2Mode, Ch3Mode, Ch4Mode>
    where
        F: FnOnce(
            GeneralTimChDrv<Tim, Int, TimCh4, DontCare>,
        ) -> GeneralTimChDrv<Tim, Int, TimCh4, Ch4Mode>;
}

#[macro_export]
macro_rules! general_tim_ch {
    ($tim_ch:ident; $trait_name:ident<$tim:ident, ..., $($modes:ident),+>.$fn_name:ident; $($ch_fields:ident),+ -> GeneralTimChDrv<$($out_modes:ident),+> for GeneralTimCfg<$($for_modes:ident),+>) => {
        impl<
            Int: drone_cortexm::thr::IntToken,
            Clk: drone_stm32f4_rcc_drv::clktree::PClkToken,
            Dir,
            Link,
            $($modes),+>
            $trait_name<$tim, Int, Clk, Dir, Link, $($modes),+> for crate::GeneralTimCfg<$tim, Int, Clk, Dir, Link, $($for_modes),+>
        {
            fn $fn_name<F, ChMode>(
                self,
                configure: F,
            ) -> crate::GeneralTimCfg<$tim, Int, Clk, Dir, Link, $($out_modes),+>
            where
                F: FnOnce(GeneralTimChDrv<$tim, Int, $tim_ch, crate::shared::DontCare>) -> GeneralTimChDrv<$tim, Int, $tim_ch, ChMode>,
            {
                let crate::GeneralTimCfg {
                    tim,
                    tim_int,
                    clk,
                    dir,
                    link,
                    cnt,
                    ovf,
                    $fn_name,
                    $($ch_fields),+
                } = self;
                let $fn_name = configure($fn_name);
                crate::GeneralTimCfg {
                    tim, tim_int, clk, dir, link, cnt, ovf, $fn_name, $($ch_fields),+
                }
            }
        }
    };
}