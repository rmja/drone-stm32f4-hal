use crate::{diverged::ExtiDiverged, ExtiLine, Syscfg, ExtiMap};
use core::marker::PhantomData;
use alloc::sync::Arc;
use drone_cortexm::{reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::{
    exti::ExtiPeriph,
};
use drone_stm32f4_gpio_drv::{GpioHeadMap, GpioPinMap, GpioPin, prelude::*};

pub trait EdgeMap: Send + Sync + 'static {}

pub struct RisingEdge;
impl EdgeMap for RisingEdge {}

pub struct FallingEdge;
impl EdgeMap for FallingEdge {}

pub struct BothEdges;
impl EdgeMap for BothEdges {}

pub struct NoEdge;
impl EdgeMap for NoEdge {}


/// EXTI driver.
pub struct ExtiDrv<
    Exti: ExtiMap,
    ExtiInt: IntToken,
    Head: GpioHeadMap,
    Edge,
> {
    pub(crate) exti: Arc<ExtiDiverged<Exti>>,
    pub(crate) exti_int: ExtiInt,
    head: PhantomData<Head>,
    edge: PhantomData<Edge>,
}

impl<
        Exti: ExtiMap,
        ExtiInt: IntToken,
        Head: GpioHeadMap,
    > ExtiDrv<Exti, ExtiInt, Head, NoEdge>
{
    /// Sets up a new [`ExtiDrv`].
    /// Syscfg is required as its clock must be enabled prior to initialization.
    pub fn new(exti: ExtiPeriph<Exti>, exti_int: ExtiInt, _syscfg: &Syscfg) -> Self {
        Self {
            exti: Arc::new(exti.into()),
            exti_int,
            head: PhantomData,
            edge: PhantomData,
        }
    }
}

impl<
        Exti: ExtiMap,
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

pub trait ExtiDrvLine<
    Exti: ExtiMap,
    ExtiInt: IntToken,
    Pin: GpioPinMap,
    PinMode: PinModeMap,
    Edge: EdgeMap,
>
{
    fn line<PinType: PinTypeMap, PinPull: PinPullMap>(
        &self,
        pin: GpioPin<Pin, PinMode, PinType, PinPull>,
    ) -> ExtiLine<Exti, ExtiInt, Pin, PinMode, PinType, PinPull, Edge>;
}

pub trait ExtiPinModes: PinModeMap {}
impl ExtiPinModes for InputMode {}
impl<Af: PinAf> ExtiPinModes for AlternateMode<Af> {}

#[macro_export]
macro_rules! exti_line {
    ($($exti:ident, $head:ident, $pin:ident;)+) => {
        $(
            impl<
                ExtiInt: drone_cortexm::thr::IntToken,
                PinMode: crate::drv::ExtiPinModes,
                Edge: crate::EdgeMap,
            >
                crate::drv::ExtiDrvLine<
                    $exti,
                    ExtiInt,
                    $pin,
                    PinMode,
                    Edge,
                > for crate::drv::ExtiDrv<$exti, ExtiInt, $head, Edge>
            {
                fn line<
                    PinType: drone_stm32f4_gpio_drv::PinTypeMap,
                    PinPull: drone_stm32f4_gpio_drv::PinPullMap,
                >(
                    &self,
                    pin: drone_stm32f4_gpio_drv::GpioPin<
                        $pin,
                        PinMode,
                        PinType,
                        PinPull,
                    >,
                ) -> crate::line::ExtiLine<$exti, ExtiInt, $pin, PinMode, PinType, PinPull, Edge> {
                    crate::line::ExtiLine::init(self, pin)
                }
            }
        )+
    };
}
