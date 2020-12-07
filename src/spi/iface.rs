use crate::master::SpiMasterDrv;
use drone_cortexm::{reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::{
    dma::ch::DmaChMap,
    gpio::pin::{GpioPinMap, GpioPinPeriph},
    spi::SpiMap,
};

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

impl<
        'drv,
        Spi: SpiMap,
        SpiInt: IntToken,
        DmaRx: DmaChMap,
        DmaRxInt: IntToken,
        DmaTx: DmaChMap,
        DmaTxInt: IntToken,
    > IfaceRoot for SpiMasterDrv<'drv, Spi, SpiInt, DmaRx, DmaRxInt, DmaTx, DmaTxInt>
{
    fn select<CsPin: GpioPinMap>(&mut self, iface: &SpiIface<CsPin>) {
        // Clear output pin by writing BR (bit reset) to the bit set/reset register.
        iface.cs.gpio_bsrr_br.set_bit();
    }

    fn deselect<CsPin: GpioPinMap>(&mut self, iface: &SpiIface<CsPin>) {
        iface.cs.gpio_bsrr_bs.set_bit();
    }
}
