use drone_cortexm::reg::prelude::*;
use drone_stm32_map::periph::gpio::head::{GpioHeadMap, GpioHeadPeriph};

pub struct GpioHead<Head: GpioHeadMap> {
    port: GpioHeadPeriph<Head>,
}

impl<Head: GpioHeadMap> GpioHead<Head> {
    /// Initialize a new gpio port head.
    #[must_use]
    pub fn with_enabled_clock(port: GpioHeadPeriph<Head>) -> Self {
        port.rcc_busenr_gpioen.set_bit();
        Self { port }
    }

    /// Disable the port clock.
    ///
    /// # Safety
    ///
    /// The port will halt when the clock is disabled.
    pub unsafe fn disable_clock(&self) {
        self.port.rcc_busenr_gpioen.clear_bit();
    }
}
