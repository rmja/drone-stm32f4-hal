use crate::periph::PwrPeriph;
use drone_cortexm::reg::prelude::*;

pub struct Pwr {
    pub pwr: PwrPeriph,
}

impl Pwr {
    #[must_use]
    pub fn init(periph: PwrPeriph) -> Pwr {
        // Enable pwr clock.
        // periph.rcc_apb1enr_pwren.set_bit();

        Pwr { pwr: periph }
    }
}

impl Pwr {
    pub fn enable_od(&self) {
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