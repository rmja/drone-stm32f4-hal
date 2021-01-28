use crate::{traits::*, periph::PwrPeriph};
use drone_cortexm::reg::prelude::*;

pub struct Pwr {
    pwr: PwrPeriph,
}

impl Pwr {
    pub fn with_enabled_clock(pwr: PwrPeriph) -> Pwr {
        // Enable pwr clock.
        pwr.rcc_apb1enr_pwren.set_bit();

        Pwr { pwr }
    }
}

// STM32F42xxx and STM32F43xxx
#[cfg(any(
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f437",
))]
impl Overdriveable for Pwr {
    fn enable_overdrive(&self) {
        // Enable the Over-drive mode and wait for the ODRDY flag to be set.
        self.pwr.pwr_cr.modify(|r| r.set_oden());
        loop {
            if self.pwr.pwr_csr.odrdy.read_bit() {
                break;
            }
        }

        // Set the ODSW bit in the PWR_CR register to switch the voltage regulator from Normal mode to Over-drive mode.
        self.pwr.pwr_cr.modify(|r| r.set_odswen());
        loop {
            if self.pwr.pwr_csr.odswrdy.read_bit() {
                break;
            }
        }
    }
}
