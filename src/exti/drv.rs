use crate::{diverged::ExtiDiverged, ExtiLine, Syscfg};
use core::marker::PhantomData;
use drone_cortexm::{reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::{
    exti::{
        ExtiFtsrFt, ExtiMap, ExtiPeriph, ExtiPrPif, ExtiRtsrRt, ExtiSwierSwi, SyscfgExticrExti,
    },
    gpio::{head::GpioHeadMap, pin::GpioPinMap},
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

/// EXTI driver.
pub struct ExtiDrv<
    Exti: ExtiMap + SyscfgExticrExti + ExtiRtsrRt + ExtiFtsrFt + ExtiSwierSwi + ExtiPrPif,
    ExtiInt: IntToken,
    Head: GpioHeadMap,
    Edge: EdgeToken,
> {
    pub(crate) exti: ExtiDiverged<Exti>,
    pub(crate) exti_int: ExtiInt,
    head: PhantomData<Head>,
    edge: PhantomData<Edge>,
}

impl<
        Exti: ExtiMap + SyscfgExticrExti + ExtiRtsrRt + ExtiFtsrFt + ExtiSwierSwi + ExtiPrPif,
        ExtiInt: IntToken,
        Head: GpioHeadMap,
    > ExtiDrv<Exti, ExtiInt, Head, NoEdge>
{
    /// Sets up a new [`ExtiDrv`].
    /// Syscfg is required as its clock must be enabled prior to initialization.
    pub fn new(exti: ExtiPeriph<Exti>, exti_int: ExtiInt, _syscfg: &Syscfg) -> Self {
        Self {
            exti: exti.into(),
            exti_int,
            head: PhantomData,
            edge: PhantomData,
        }
    }
}

impl<
        Exti: ExtiMap + SyscfgExticrExti + ExtiRtsrRt + ExtiFtsrFt + ExtiSwierSwi + ExtiPrPif,
        ExtiInt: IntToken,
        Head: GpioHeadMap,
    > ExtiDrv<Exti, ExtiInt, Head, NoEdge>
{
    pub fn into_rising_edge(self) -> ExtiDrv<Exti, ExtiInt, Head, RisingEdge> {
        self.exti.exti_rtsr_rt.set_bit(); // rising trigger enabled
        ExtiDrv {
            exti: self.exti,
            exti_int: self.exti_int,
            head: PhantomData,
            edge: PhantomData,
        }
    }

    pub fn into_falling_edge(self) -> ExtiDrv<Exti, ExtiInt, Head, FallingEdge> {
        self.exti.exti_ftsr_ft.set_bit(); // falling trigger enabled
        ExtiDrv {
            exti: self.exti,
            exti_int: self.exti_int,
            head: PhantomData,
            edge: PhantomData,
        }
    }

    pub fn into_both_edges(self) -> ExtiDrv<Exti, ExtiInt, Head, BothEdges> {
        self.exti.exti_rtsr_rt.set_bit(); // rising trigger enabled
        self.exti.exti_ftsr_ft.set_bit(); // falling trigger enabled
        ExtiDrv {
            exti: self.exti,
            exti_int: self.exti_int,
            head: PhantomData,
            edge: PhantomData,
        }
    }
}

pub trait AnyEdge: EdgeToken {}
impl AnyEdge for RisingEdge {}
impl AnyEdge for FallingEdge {}
impl AnyEdge for BothEdges {}

impl<
        Exti: ExtiMap + SyscfgExticrExti + ExtiRtsrRt + ExtiFtsrFt + ExtiSwierSwi + ExtiPrPif,
        ExtiInt: IntToken,
        Head: GpioHeadMap,
        Edge: AnyEdge,
    > ExtiDrv<Exti, ExtiInt, Head, Edge>
{
    pub fn listen(&self) {
        self.exti.exti_imr_im.set_bit(); // Unmask interrupt request
    }

    pub fn unlisten(&self) {
        self.exti.exti_imr_im.clear_bit(); // Mask interrupt request
    }
}

pub trait ExtiDrvLine<
    Exti: ExtiMap + SyscfgExticrExti + ExtiRtsrRt + ExtiFtsrFt + ExtiSwierSwi + ExtiPrPif,
    ExtiInt: IntToken,
    Edge: EdgeToken,
    Mode: PinModeToken,
    Pin: GpioPinMap,
>
{
    fn line<Type: PinTypeToken, Pull: PinPullToken>(
        &self,
        pin: GpioPin<Pin, Mode, Type, Pull>,
    ) -> ExtiLine<Exti, ExtiInt, Edge>;
}

#[macro_export]
macro_rules! exti_line {
    ($exti:ident, $head:ident, $pin:ident) => {
        impl<ExtiInt: drone_cortexm::thr::IntToken, Edge: EdgeToken>
            crate::drv::ExtiDrvLine<
                $exti,
                ExtiInt,
                Edge,
                drone_stm32f4_gpio_drv::prelude::InputMode,
                $pin,
            > for ExtiDrv<$exti, ExtiInt, $head, Edge>
        {
            fn line<
                Type: drone_stm32f4_gpio_drv::PinTypeToken,
                Pull: drone_stm32f4_gpio_drv::PinPullToken,
            >(
                &self,
                _pin: drone_stm32f4_gpio_drv::GpioPin<
                    $pin,
                    drone_stm32f4_gpio_drv::prelude::InputMode,
                    Type,
                    Pull,
                >,
            ) -> crate::line::ExtiLine<$exti, ExtiInt, Edge> {
                crate::line::ExtiLine::init(self)
            }
        }

        impl<
                ExtiInt: drone_cortexm::thr::IntToken,
                Edge: EdgeToken,
                Af: drone_stm32f4_gpio_drv::prelude::PinAfToken,
            >
            crate::drv::ExtiDrvLine<
                $exti,
                ExtiInt,
                Edge,
                drone_stm32f4_gpio_drv::prelude::AlternateMode<Af>,
                $pin,
            > for ExtiDrv<$exti, ExtiInt, $head, Edge>
        {
            fn line<
                Type: drone_stm32f4_gpio_drv::PinTypeToken,
                Pull: drone_stm32f4_gpio_drv::PinPullToken,
            >(
                &self,
                _pin: drone_stm32f4_gpio_drv::GpioPin<
                    $pin,
                    drone_stm32f4_gpio_drv::prelude::AlternateMode<Af>,
                    Type,
                    Pull,
                >,
            ) -> crate::line::ExtiLine<$exti, ExtiInt, Edge> {
                crate::line::ExtiLine::init(self)
            }
        }
    };
}
