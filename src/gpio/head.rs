use drone_cortexm::reg::prelude::*;
use drone_stm32_map::periph::gpio::head::{GpioHeadMap,GpioHeadPeriph};

pub struct GpioHead<Gpio: GpioHeadMap> {
    port: GpioHeadPeriph<Gpio>,
}

impl<Gpio: GpioHeadMap> GpioHead<Gpio> {
    /// Initialize a new gpio port head.
    #[must_use]
    pub fn init(port: GpioHeadPeriph<Gpio>) -> GpioHead<Gpio> {
        port.rcc_busenr_gpioen.set_bit();
        Self { port }
    }

    /// Disable the port clock.
    pub unsafe fn disable_clk(self) {
        self.port.rcc_busenr_gpioen.clear_bit();
    }
}