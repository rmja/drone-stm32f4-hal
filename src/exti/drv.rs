use crate::{ExtiLine, Syscfg, diverged::ExtiDiverged};
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
    /// Create a new exti setup.
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
    pub(crate) exti: ExtiDiverged<Exti>,
    pub(crate) exti_int: ExtiInt,
    edge: PhantomData<Edge>,
}

impl<
        Exti: ExtiMap + SyscfgExticrExti + ExtiRtsrRt + ExtiFtsrFt + ExtiSwierSwi + ExtiPrPif,
        ExtiInt: IntToken,
    > ExtiDrv<Exti, ExtiInt, NoEdge>
{
    /// Sets up a new [`ExtiDrv`] from `setup` values.
    /// Syscfg is required as its clock must be enabled prior to initialization.
    pub fn init(setup: ExtiSetup<Exti, ExtiInt>, _syscfg: &Syscfg) -> ExtiDrv<Exti, ExtiInt, NoEdge> {
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
        self.exti.exti_imr_im.set_bit(); // Unmask interrupt request
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

pub trait ExtiDrvLine<
    Exti: ExtiMap + SyscfgExticrExti + ExtiRtsrRt + ExtiFtsrFt + ExtiSwierSwi + ExtiPrPif,
    ExtiInt: IntToken,
    Pin: GpioPinMap,
>
{
    fn line<Type: PinTypeToken, Pull: PinPullToken>(
        &self,
        pin: &GpioPin<Pin, InputMode, Type, Pull>,
    ) -> ExtiLine<Exti, ExtiInt>;
}

#[macro_export]
macro_rules! exti_line {
    ($exti:ident, $port:ident, $pin:ident) => {
        impl<
            ExtiInt: drone_cortexm::thr::IntToken,
            Edge: EdgeToken,
        > crate::drv::ExtiDrvLine<
            $exti,
            ExtiInt,
            $pin
        > for ExtiDrv<$exti, ExtiInt, Edge>
        {
            fn line<
                Type: drone_stm32f4_gpio_drv::PinTypeToken,
                Pull: drone_stm32f4_gpio_drv::PinPullToken,
            >(
                &self,
                _pin: &drone_stm32f4_gpio_drv::GpioPin<
                    $pin,
                    drone_stm32f4_gpio_drv::prelude::InputMode,
                    Type,
                    Pull,
                >,
            ) -> crate::line::ExtiLine<$exti, ExtiInt> {
                crate::line::ExtiLine::init(self, $port::num())
            }
        }
    };
}