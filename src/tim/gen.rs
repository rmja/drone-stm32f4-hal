use core::{marker::PhantomData, num::NonZeroUsize};

use alloc::rc::Rc;
use drone_core::fib;
use drone_cortexm::{reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::tim::general::{
    traits::*, GeneralTimMap, GeneralTimPeriph, TimCr1Cms, TimCr1Dir,
};
use drone_stm32f4_rcc_drv::{clktree::*, traits::ConfiguredClk};
use futures::Stream;

use crate::{
    gen_ch::{ModeToken, TimCh1, TimCh2, TimChCfg},
    shared::DontCare,
    TimCh3, TimCh4, TimFreq,
};

pub struct DirCountUp;
pub struct DirCountDown;

pub trait DirToken {}

impl DirToken for DirCountUp {}
impl DirToken for DirCountDown {}

pub struct GeneralTimSetup<Tim: GeneralTimMap, Int: IntToken, Clk: PClkToken> {
    pub tim: GeneralTimPeriph<Tim>,
    pub tim_int: Int,
    pub clk: ConfiguredClk<Clk>,
    pub freq: TimFreq,
    pub arr: u32,
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
    Ch1Mode,
    Ch2Mode,
    Ch3Mode,
    Ch4Mode,
> {
    pub(crate) tim: Rc<GeneralTimPeriph<Tim>>,
    pub(crate) tim_int: Int,
    pub(crate) clk: ConfiguredClk<Clk>,
    pub(crate) dir: PhantomData<Dir>,
    pub ch1: TimChCfg<Tim, Int, TimCh1, Ch1Mode>,
    pub ch2: TimChCfg<Tim, Int, TimCh2, Ch2Mode>,
    pub ch3: TimChCfg<Tim, Int, TimCh3, Ch3Mode>,
    pub ch4: TimChCfg<Tim, Int, TimCh4, Ch4Mode>,
}

// pub struct GeneralTimDiverged<Tim: GeneralTimMap> {
//     pub(crate) tim_cr1: Tim::STimCr1,
//     pub(crate) tim_dier: Tim::CTimDier,
//     pub(crate) tim_sr: Tim::CTimSr,
//     pub(crate) tim_arr: Tim::STimArr,
//     pub(crate) tim_egr: Tim::STimEgr,
//     pub(crate) tim_cnt: Tim::CTimCnt,
// }

impl<Tim: GeneralTimMap, Int: IntToken, Clk: PClkToken>
    GeneralTimCfg<Tim, Int, Clk, DontCare, DontCare, DontCare, DontCare, DontCare>
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
        } = setup;

        // Enable clock.
        tim.rcc_busenr_timen.set_bit();

        // Set prescaler
        tim.tim_psc.psc().write_bits(tim_psc(&clk, freq) as u32);

        // Set some sensible register values.
        tim.tim_cr1.store_reg(|r, v| {
            r.udis().clear(v); // Enable counter overflow event generation
            r.urs().set(v); // Only counter overflow generates an update interrupt
            r.opm().clear(v); // Counter is not stopped at update event
                              // dir and cms are set for count direction is configured.
            r.arpe().set(v) // Use buffered auto reload value
        });

        // Set the auto-reload register to a full period.
        // This defines the number of bits in the timer.
        tim.tim_arr.arr().write_bits(arr);

        // Re-initialize the counter and generate an update of the registers.
        tim.tim_egr.ug().set_bit();

        // let tim = Rc::new(GeneralTimDiverged {
        //     tim_cr1: tim.tim_cr1,
        //     tim_dier: tim.tim_dier.into_copy(),
        //     tim_sr: tim.tim_sr.into_copy(),
        //     tim_arr: tim.tim_arr,
        //     tim_egr: tim.tim_egr,
        //     tim_cnt: tim.tim_cnt.into_copy(),
        // });
        let tim = Rc::new(tim);
        Self {
            tim: tim.clone(),
            tim_int,
            clk,
            dir: PhantomData,
            ch1: TimChCfg::new(tim.clone(), tim_int),
            ch2: TimChCfg::new(tim.clone(), tim_int),
            ch3: TimChCfg::new(tim.clone(), tim_int),
            ch4: TimChCfg::new(tim.clone(), tim_int),
        }
    }
}

impl<
        Tim: GeneralTimMap + TimCr1Dir + TimCr1Cms,
        Int: IntToken,
        Clk: PClkToken,
        Ch1Mode,
        Ch2Mode,
        Ch3Mode,
        Ch4Mode,
    > GeneralTimCfg<Tim, Int, Clk, DontCare, Ch1Mode, Ch2Mode, Ch3Mode, Ch4Mode>
{
    pub fn into_count_up(
        self,
    ) -> GeneralTimCfg<Tim, Int, Clk, DirCountUp, Ch1Mode, Ch2Mode, Ch3Mode, Ch4Mode> {
        let GeneralTimCfg {
            tim,
            tim_int,
            clk,
            ch1,
            ch2,
            ch3,
            ch4,
            ..
        } = self;

        tim.tim_cr1.store_reg(|r, v| {
            r.dir().clear(v); // Count up
            r.cms().write(v, 0b00); // Count up or down depending on the direction bit (i.e. count up)
        });

        GeneralTimCfg {
            tim,
            tim_int,
            clk,
            dir: PhantomData,
            ch1,
            ch2,
            ch3,
            ch4,
        }
    }

    pub fn into_count_down(
        self,
    ) -> GeneralTimCfg<Tim, Int, Clk, DirCountDown, Ch1Mode, Ch2Mode, Ch3Mode, Ch4Mode> {
        let GeneralTimCfg {
            tim,
            tim_int,
            clk,
            ch1,
            ch2,
            ch3,
            ch4,
            ..
        } = self;

        tim.tim_cr1.store_reg(|r, v| {
            r.dir().set(v); // Count down
            r.cms().write(v, 0b00); // Count up or down depending on the direction bit (i.e. count down)
        });

        GeneralTimCfg {
            tim,
            tim_int,
            clk,
            dir: PhantomData,
            ch1,
            ch2,
            ch3,
            ch4,
        }
    }
}

pub trait ConfigureTimCh1<
    Tim: GeneralTimMap,
    Int: IntToken,
    Clk: PClkToken,
    Dir: DirToken,
    Ch2Mode,
    Ch3Mode,
    Ch4Mode,
>
{
    fn ch1<F, Ch1Mode: ModeToken>(
        self,
        configure: F,
    ) -> GeneralTimCfg<Tim, Int, Clk, Dir, Ch1Mode, Ch2Mode, Ch3Mode, Ch4Mode>
    where
        F: FnOnce(TimChCfg<Tim, Int, TimCh1, DontCare>) -> TimChCfg<Tim, Int, TimCh1, Ch1Mode>;
}

pub trait ConfigureTimCh2<
    Tim: GeneralTimMap,
    Int: IntToken,
    Clk: PClkToken,
    Dir: DirToken,
    Ch1Mode,
    Ch3Mode,
    Ch4Mode,
>
{
    fn ch2<F, Ch2Mode: ModeToken>(
        self,
        configure: F,
    ) -> GeneralTimCfg<Tim, Int, Clk, Dir, Ch1Mode, Ch2Mode, Ch3Mode, Ch4Mode>
    where
        F: FnOnce(TimChCfg<Tim, Int, TimCh2, DontCare>) -> TimChCfg<Tim, Int, TimCh2, Ch2Mode>;
}

pub trait ConfigureTimCh3<
    Tim: GeneralTimMap,
    Int: IntToken,
    Clk: PClkToken,
    Dir: DirToken,
    Ch1Mode,
    Ch2Mode,
    Ch4Mode,
>
{
    fn ch3<F, Ch3Mode: ModeToken>(
        self,
        configure: F,
    ) -> GeneralTimCfg<Tim, Int, Clk, Dir, Ch1Mode, Ch2Mode, Ch3Mode, Ch4Mode>
    where
        F: FnOnce(TimChCfg<Tim, Int, TimCh3, DontCare>) -> TimChCfg<Tim, Int, TimCh3, Ch3Mode>;
}

pub trait ConfigureTimCh4<
    Tim: GeneralTimMap,
    Int: IntToken,
    Clk: PClkToken,
    Dir: DirToken,
    Ch1Mode,
    Ch2Mode,
    Ch3Mode,
>
{
    fn ch4<F, Ch4Mode: ModeToken>(
        self,
        configure: F,
    ) -> GeneralTimCfg<Tim, Int, Clk, Dir, Ch1Mode, Ch2Mode, Ch3Mode, Ch4Mode>
    where
        F: FnOnce(TimChCfg<Tim, Int, TimCh4, DontCare>) -> TimChCfg<Tim, Int, TimCh4, Ch4Mode>;
}

#[macro_export]
macro_rules! general_tim_ch {
    ($tim_ch:ident; $trait_name:ident<$tim:ident, ..., $($modes:ident),+>.$fn_name:ident; $($ch_fields:ident),+ -> TimChCfg<$($out_modes:ident),+> for GeneralTimCfg<$($for_modes:ident),+>) => {
        impl<Int: drone_cortexm::thr::IntToken, Clk: drone_stm32f4_rcc_drv::clktree::PClkToken, Dir: crate::DirToken, $($modes),+>
            $trait_name<$tim, Int, Clk, Dir, $($modes),+> for crate::GeneralTimCfg<$tim, Int, Clk, Dir, $($for_modes),+>
        {
            fn $fn_name<F, Mode: crate::ModeToken>(
                self,
                configure: F,
            ) -> crate::GeneralTimCfg<$tim, Int, Clk, Dir, $($out_modes),+>
            where
                F: FnOnce(TimChCfg<$tim, Int, $tim_ch, crate::shared::DontCare>) -> TimChCfg<$tim, Int, $tim_ch, Mode>,
            {
                let crate::GeneralTimCfg {
                    tim,
                    tim_int,
                    clk,
                    dir,
                    $fn_name,
                    $($ch_fields),+
                } = self;
                let $fn_name = configure($fn_name);
                crate::GeneralTimCfg {
                    tim, tim_int, clk, dir, $fn_name, $($ch_fields),+
                }
            }
        }
    };
}

impl<Tim: GeneralTimMap, Int: IntToken, Clk: PClkToken, Dir: DirToken, Ch1Mode, Ch2Mode, Ch3Mode, Ch4Mode>
    GeneralTimCfg<Tim, Int, Clk, Dir, Ch1Mode, Ch2Mode, Ch3Mode, Ch4Mode>
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

    /// Get the current counter value.
    pub fn counter(&self) -> u32 {
        self.tim.tim_cnt.cnt().read_bits() as u32
    }

    // pub fn overflow_saturating_pulse_stream(&self) -> impl Stream<Item = NonZeroUsize> {

    //     self.tim_int.add_saturating_pulse_stream(fib::new_fn(move || {
    //         if self.is_pending_overflow() {
    //             self.clear_pending_overflow();
    //             fib::Yielded(Some(1))
    //         }
    //         else {
    //             fib::Yielded(None)
    //         }
    //     }))
    // }

    /// Get the overflow pending flag.
    pub fn is_pending_overflow(&self) -> bool {
        self.tim.tim_sr.uif().read_bit()
    }

    /// Clear the overflow pending flag.
    pub fn clear_pending_overflow(&self) {
        // rc_w0: Clear flag by writing a 0, 1 has no effect.
        let mut val = unsafe { Tim::STimSr::val_from(u32::MAX) };
        self.tim.tim_sr.uif().clear(&mut val);
        self.tim.tim_sr.store_val(val);
    }

    /// Release the timer peripheral.
    pub fn release(self) -> GeneralTimPeriph<Tim> {
        let Self { tim, .. } = self;
        match Rc::try_unwrap(tim) {
            Ok(tim) => tim,
            Err(_) => unreachable!(),
        }
    }
}

fn tim_psc<Clk: PClkToken>(clk: &ConfiguredClk<Clk>, freq: TimFreq) -> u16 {
    let f_pclk_timer = clk.freq() * 2; // The PCLK is multipled by 2 before it enters the timer, see the clock tree for reference.
    match freq {
        TimFreq::Nominal(freq) => (((f_pclk_timer + (freq / 2)) / freq) - 1) as u16,
        TimFreq::Prescaler(prescaler) => prescaler - 1,
    }
}
