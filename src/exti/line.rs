use crate::{ExtiDrv, FallingEdge, diverged::ExtiDiverged};
use core::{marker::PhantomData, pin::Pin, task::{Context, Poll}};
use displaydoc::Display;
use drone_core::fib::{FiberStreamPulse, TryFiberStreamPulse};
use drone_cortexm::{fib, fib::Fiber, reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::{
    exti::{ExtiFtsrFt, ExtiMap, ExtiPrPif, ExtiRtsrRt, ExtiSwierSwi, SyscfgExticrExti},
    gpio::head::GpioHeadMap,
};
use drone_stm32_map::periph::gpio::pin::GpioPinMap;
use drone_stm32f4_gpio_drv::{GpioPin, prelude::*};
use futures::{future, prelude::*};

/// EXTI stream overflow
#[derive(Display, Debug)]
pub struct ExtiOverflow;

pub trait HeadNum {
    const NUM: u32;
}

pub struct ExtiLine<
    'drv,
    Exti: ExtiMap + SyscfgExticrExti + ExtiRtsrRt + ExtiFtsrFt + ExtiSwierSwi + ExtiPrPif,
    ExtiInt: IntToken,
    Pin: GpioPinMap, PinMode, PinType, PinPull,
    Edge: 'static,
> {
    exti: &'drv ExtiDiverged<Exti>,
    exti_int: ExtiInt,
    pub pin: GpioPin<Pin, PinMode, PinType, PinPull>,
    edge: PhantomData<Edge>,
}

impl<
        'drv,
        Exti: ExtiMap + SyscfgExticrExti + ExtiRtsrRt + ExtiFtsrFt + ExtiSwierSwi + ExtiPrPif,
        ExtiInt: IntToken,
        Pin: GpioPinMap, PinMode: 'static, PinType: 'static, PinPull: 'static,
        Edge,
    > ExtiLine<'drv, Exti, ExtiInt, Pin, PinMode, PinType, PinPull, Edge>
{
    pub(crate) fn init<Head: GpioHeadMap + HeadNum>(
        exti: &'drv ExtiDrv<Exti, ExtiInt, Head, Edge>,
        pin: GpioPin<Pin, PinMode, PinType, PinPull>,
    ) -> Self {
        exti.exti.syscfg_exticr_exti.write_bits(Head::NUM);
        Self {
            exti: &exti.exti,
            exti_int: exti.exti_int,
            pin,
            edge: PhantomData,
        }
    }

    /// Creates a new saturating stream of external events.
    pub fn saturating_pulse_stream(&self) -> FiberStreamPulse {
        self.exti_int.add_saturating_pulse_stream(self.new_fib())
    }

    /// Creates a new fallible stream of external events.
    pub fn pulse_try_stream(
        &self,
    ) -> TryFiberStreamPulse<ExtiOverflow> {
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

impl<
        'drv,
        Exti: ExtiMap + SyscfgExticrExti + ExtiRtsrRt + ExtiFtsrFt + ExtiSwierSwi + ExtiPrPif,
        ExtiInt: IntToken,
        Pin: GpioPinMap, PinMode: PinGetMode + 'static, PinType: 'static, PinPull: 'static,
    > ExtiLine<'drv, Exti, ExtiInt, Pin, PinMode, PinType, PinPull, FallingEdge>
{
    /// Wait for the line to become low. Return immediately if this is already the case.
    pub fn wait_low(&self) -> WaitFuture {
        // Register interrupt
        let exti_pr_pif = self.exti.exti_pr_pif;
        let exti_imr_im = self.exti.exti_imr_im;
        let future = self.exti_int.add_future(fib::new_fn(move || {
            if exti_pr_pif.read_bit() {
                // Selected trigger request occurred
                exti_imr_im.clear_bit(); // Disable interrupt
                fib::Complete(())
            } else {
                fib::Yielded(())
            }
        }));

        exti_pr_pif.set_bit(); // Clear pending flag
        exti_imr_im.set_bit(); // Enable interrupt

        // Only wait for falling interrupt if we are currently high
        if self.pin.get() {
            WaitFuture(Box::pin(future))
        }
        else {
            exti_imr_im.clear_bit(); // Disable interrupt
            WaitFuture(Box::pin(future::ready(())))
        }
    }
}

pub struct WaitFuture<'a>(Pin<Box<dyn Future<Output = ()> + Send + 'a>>);

impl Future for WaitFuture<'_> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.0.as_mut().poll(cx)
    }
}