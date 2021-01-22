use crate::{diverged::ExtiDiverged, drv::EdgeToken, ExtiDrv};
use core::{marker::PhantomData, num::NonZeroUsize};
use displaydoc::Display;
use drone_cortexm::{fib, fib::Fiber, reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::{
    exti::{ExtiFtsrFt, ExtiMap, ExtiPrPif, ExtiRtsrRt, ExtiSwierSwi, SyscfgExticrExti},
    gpio::head::GpioHeadMap,
};
use futures::Stream;

/// EXTI stream overflow
#[derive(Display, Debug)]
pub struct ExtiOverflow;

pub trait HeadNum {
    fn num() -> u32;
}

pub struct ExtiLine<
    'drv,
    Exti: ExtiMap + SyscfgExticrExti + ExtiRtsrRt + ExtiFtsrFt + ExtiSwierSwi + ExtiPrPif,
    ExtiInt: IntToken,
    Edge: EdgeToken + 'static,
> {
    exti: &'drv ExtiDiverged<Exti>,
    exti_int: ExtiInt,
    edge: PhantomData<Edge>,
}

impl<
        'drv,
        Exti: ExtiMap + SyscfgExticrExti + ExtiRtsrRt + ExtiFtsrFt + ExtiSwierSwi + ExtiPrPif,
        ExtiInt: IntToken,
        Edge: EdgeToken,
    > ExtiLine<'drv, Exti, ExtiInt, Edge>
{
    pub(crate) fn init<Head: GpioHeadMap + HeadNum>(
        exti: &'drv ExtiDrv<Exti, ExtiInt, Head, Edge>,
    ) -> Self {
        exti.exti.syscfg_exticr_exti.write_bits(Head::num());
        Self {
            exti: &exti.exti,
            exti_int: exti.exti_int,
            edge: PhantomData,
        }
    }

    /// Creates a new saturating stream of external events.
    pub fn saturating_pulse_stream(&self) -> impl Stream<Item = NonZeroUsize> + Send + Sync {
        self.exti_int.add_saturating_pulse_stream(self.new_fib())
    }

    /// Creates a new fallible stream of external events.
    pub fn pulse_try_stream(
        &self,
    ) -> impl Stream<Item = Result<NonZeroUsize, ExtiOverflow>> + Send + Sync {
        self.exti_int
            .add_pulse_try_stream(|| Err(ExtiOverflow), self.new_fib())
    }

    fn new_fib<R>(&self) -> impl Fiber<Input = (), Yield = Option<usize>, Return = R> {
        let exti_pr_pif = self.exti.exti_pr_pif;
        fib::new_fn(move || {
            if exti_pr_pif.read_bit() {
                // Selected trigger request occurred: clear pending flag
                exti_pr_pif.set_bit();
                fib::Yielded(Some(1))
            } else {
                fib::Yielded(None)
            }
        })
    }
}
