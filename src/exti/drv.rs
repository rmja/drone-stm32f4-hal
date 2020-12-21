use crate::diverged::ExtiDiverged;
use core::marker::PhantomData;
use displaydoc::Display;
use drone_cortexm::{fib, fib::Fiber, reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::{
    exti::{
        ExtiFtsrFt, ExtiMap, ExtiPeriph, ExtiPrPif, ExtiRtsrRt, ExtiSwierSwi, SyscfgExticrExti,
    },
    gpio::pin::GpioPinMap,
};
use drone_stm32f4_gpio_drv::{prelude::*, GpioPin};
// use futures::prelude::*;

pub struct RisingEdge;
pub struct FallingEdge;
pub struct BothEdges;
pub struct NoEdge;

pub trait EdgeToken {}

impl EdgeToken for RisingEdge {}

impl EdgeToken for FallingEdge {}

impl EdgeToken for BothEdges {}

impl EdgeToken for NoEdge {}

/// EXTI stream overflow
#[derive(Display, Debug)]
pub struct ExtiOverflow;

/// EXTI setup.
pub struct ExtiSetup<
    Exti: ExtiMap + SyscfgExticrExti + ExtiRtsrRt + ExtiFtsrFt + ExtiSwierSwi + ExtiPrPif,
    ExtiInt: IntToken,
> {
    /// EXTI peripheral.
    pub exti: ExtiPeriph<Exti>,
    /// EXTI interrupt.
    pub exti_int: ExtiInt,
}

impl<
    Exti: ExtiMap + SyscfgExticrExti + ExtiRtsrRt + ExtiFtsrFt + ExtiSwierSwi + ExtiPrPif,
    ExtiInt: IntToken,
> ExtiSetup<Exti, ExtiInt> {
    pub fn new(exti: ExtiPeriph<Exti>, exti_int: ExtiInt) -> Self {
        Self {
            exti,
            exti_int,
        }
    }
}

/// EXTI driver.
pub struct ExtiDrv<
    Exti: ExtiMap + SyscfgExticrExti + ExtiRtsrRt + ExtiFtsrFt + ExtiSwierSwi + ExtiPrPif,
    ExtiInt: IntToken,
    Edge: EdgeToken,
> {
    exti: ExtiDiverged<Exti>,
    exti_int: ExtiInt,
    edge: PhantomData<Edge>,
}

pub struct ExtiLineDrv<
    'drv,
    Exti: ExtiMap + SyscfgExticrExti + ExtiRtsrRt + ExtiFtsrFt + ExtiSwierSwi + ExtiPrPif,
    ExtiInt: IntToken,
> {
    exti: &'drv ExtiDiverged<Exti>,
    exti_int: ExtiInt,
}

impl<
        Exti: ExtiMap + SyscfgExticrExti + ExtiRtsrRt + ExtiFtsrFt + ExtiSwierSwi + ExtiPrPif,
        ExtiInt: IntToken,
    > ExtiDrv<Exti, ExtiInt, NoEdge>
{
    /// Sets up a new [`ExtiDrv`] from `setup` values.
    pub fn init(setup: ExtiSetup<Exti, ExtiInt>) -> ExtiDrv<Exti, ExtiInt, NoEdge> {
        let ExtiSetup { exti, exti_int } = setup;
        let drv = ExtiDrv {
            exti: exti.into(),
            exti_int,
            edge: PhantomData,
        };
        drv.init_exti();
        drv
    }

    fn init_exti(&self) {
        self.exti.exti_imr_im.set_bit(); // interrupt request from line 4 is not masked
    }
}

impl<
        Exti: ExtiMap + SyscfgExticrExti + ExtiRtsrRt + ExtiFtsrFt + ExtiSwierSwi + ExtiPrPif,
        ExtiInt: IntToken,
    > ExtiDrv<Exti, ExtiInt, NoEdge>
{
    pub fn into_rising_edge(self) -> ExtiDrv<Exti, ExtiInt, RisingEdge> {
        self.exti.exti_rtsr_rt.set_bit(); // rising trigger enabled
        ExtiDrv {
            exti: self.exti,
            exti_int: self.exti_int,
            edge: PhantomData,
        }
    }

    pub fn into_falling_edge(self) -> ExtiDrv<Exti, ExtiInt, FallingEdge> {
        self.exti.exti_ftsr_ft.set_bit(); // falling trigger enabled
        ExtiDrv {
            exti: self.exti,
            exti_int: self.exti_int,
            edge: PhantomData,
        }
    }

    pub fn into_both_edges(self) -> ExtiDrv<Exti, ExtiInt, BothEdges> {
        self.exti.exti_rtsr_rt.set_bit(); // rising trigger enabled
        self.exti.exti_ftsr_ft.set_bit(); // falling trigger enabled
        ExtiDrv {
            exti: self.exti,
            exti_int: self.exti_int,
            edge: PhantomData,
        }
    }
}

pub trait ExtiLine<
    Exti: ExtiMap + SyscfgExticrExti + ExtiRtsrRt + ExtiFtsrFt + ExtiSwierSwi + ExtiPrPif,
    ExtiInt: IntToken,
    Pin: GpioPinMap,
>
{
    fn line<Type: PinTypeToken, Pull: PinPullToken>(
        &self,
        pin: GpioPin<Pin, InputMode, Type, Pull>,
    ) -> ExtiLineDrv<Exti, ExtiInt>;
}

impl<
        'drv,
        Exti: ExtiMap + SyscfgExticrExti + ExtiRtsrRt + ExtiFtsrFt + ExtiSwierSwi + ExtiPrPif,
        ExtiInt: IntToken,
    > ExtiLineDrv<'drv, Exti, ExtiInt>
{
    pub(crate) fn init<Edge: EdgeToken>(exti: &'drv ExtiDrv<Exti, ExtiInt, Edge>) -> Self {
        // self.exti.syscfg_exticr_exti.write_bits(config); // configuration
        Self {
            exti: &exti.exti,
            exti_int: exti.exti_int,
        }
    }

    // pub fn add_fib(&self) -> u32 {
    //     self.exti_int.add_future
    // }

    /// Creates a new saturating stream of external events.
    // pub fn create_saturating_stream(&self) -> impl Stream<Item = NonZeroUsize> + Send + Sync {
    //     self.exti_int.add_saturating_pulse_stream(self.new_fib())
    // }

    // /// Creates a new fallible stream of external events.
    // pub fn create_try_stream(
    //     &self,
    // ) -> impl Stream<Item = Result<NonZeroUsize, ExtiOverflow>> + Send + Sync {
    //     self.exti_int
    //         .add_pulse_try_stream(|| Err(ExtiOverflow), self.new_fib())
    // }

    fn new_fib<R>(&self) -> impl Fiber<Input = (), Yield = Option<usize>, Return = R> {
        let exti_pr_pif = self.exti.exti_pr_pif;
        fib::new_fn(move || {
            if exti_pr_pif.read_bit() {
                // selected trigger request occurred
                exti_pr_pif.set_bit();
                fib::Yielded(Some(1))
            } else {
                fib::Yielded(None)
            }
        })
    }
}
