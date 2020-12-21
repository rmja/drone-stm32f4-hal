use crate::drv::{EdgeToken, ExtiDrv, ExtiLine};
use drone_stm32_map::periph::{exti::Exti4, gpio::pin::*};

impl<ExtiInt: drone_cortexm::thr::IntToken, Edge: EdgeToken> ExtiLine<Exti4, ExtiInt, GpioB4>
    for ExtiDrv<Exti4, ExtiInt, Edge>
{
    fn line<
        Type: drone_stm32f4_gpio_drv::PinTypeToken,
        Pull: drone_stm32f4_gpio_drv::PinPullToken,
    >(
        &self,
        pin: drone_stm32f4_gpio_drv::GpioPin<
            GpioB4,
            drone_stm32f4_gpio_drv::prelude::InputMode,
            Type,
            Pull,
        >,
    ) -> crate::drv::ExtiLineDrv<Exti4, ExtiInt> {
        crate::drv::ExtiLineDrv::init(self)
    }
}
