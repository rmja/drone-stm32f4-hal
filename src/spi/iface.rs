use crate::master::SpiMasterDrv;
use drone_cortexm::reg::prelude::*;
use drone_stm32_map::periph::gpio::pin::{GpioPinMap, GpioPinPeriph};

pub struct SpiIface<CsPin: GpioPinMap> {
    cs: GpioPinPeriph<CsPin>,
}

pub trait IfaceRoot {
    fn select<CsPin: GpioPinMap>(&mut self, iface: &SpiIface<CsPin>);
    fn deselect<CsPin: GpioPinMap>(&mut self, iface: &SpiIface<CsPin>);
}

impl<CsPin: GpioPinMap> SpiIface<CsPin> {
    pub fn new(cs: GpioPinPeriph<CsPin>) -> Self {
        Self { cs }
    }
}

impl IfaceRoot for SpiMasterDrv {
    fn select<CsPin: GpioPinMap>(&mut self, iface: &SpiIface<CsPin>) {
        // Clear output pin by writing BR (bit reset) to the bit set/reset register.
        iface.cs.gpio_bsrr_br.set_bit();
    }

    fn deselect<CsPin: GpioPinMap>(&mut self, iface: &SpiIface<CsPin>) {
        iface.cs.gpio_bsrr_bs.set_bit();
    }
}