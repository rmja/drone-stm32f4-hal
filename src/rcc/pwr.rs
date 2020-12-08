use drone_cortexm::reg::prelude::*;
use drone_stm32_map::reg;

pub struct Pwr {
    /// The power control register.
    pub pwr_cr: reg::pwr::Cr<Srt>,
    /// The power status register.
    pub pwr_csr: reg::pwr::Csr<Srt>,
}

impl Pwr {
    pub fn enable_od(&self) {
        // Enable the Over-drive mode and wait for the ODRDY flag to be set.
        self.pwr_cr.modify(|r| r.set_oden());
        loop {
            if self.pwr_csr.odrdy.read_bit() {
                break;
            }
        }

        // Set the ODSW bit in the PWR_CR register to switch the voltage regulator from Normal mode to Over-drive mode.
        self.pwr_cr.modify(|r| r.set_odswen());
        loop {
            if self.pwr_csr.odswrdy.read_bit() {
                break;
            }
        }
    }
}