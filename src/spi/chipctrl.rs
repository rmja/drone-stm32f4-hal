use crate::master::SpiMasterDrv;
use drone_cortexm::{reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::{
    dma::ch::DmaChMap,
    gpio::pin::{GpioPinMap, GpioPinPeriph},
    spi::SpiMap,
};

pub struct SpiChip<CsPin: GpioPinMap> {
    cs: GpioPinPeriph<CsPin>,
}

impl<CsPin: GpioPinMap> SpiChip<CsPin> {
    pub fn init(cs: GpioPinPeriph<CsPin>) -> Self {
        // Set output pin by writing BS (bit set) to the bit set/reset register.
        cs.gpio_bsrr_bs.set_bit();
        Self { cs }
    }
}

pub trait ChipCtrl {
    fn select<CsPin: GpioPinMap>(&mut self, chip: &SpiChip<CsPin>);
    fn deselect<CsPin: GpioPinMap>(&mut self, chip: &SpiChip<CsPin>);
}

impl<
    'drv,
    Spi: SpiMap,
    SpiInt: IntToken,
    DmaRx: DmaChMap,
    DmaRxInt: IntToken,
    DmaTx: DmaChMap,
    DmaTxInt: IntToken,
> ChipCtrl for SpiMasterDrv<'drv, Spi, SpiInt, DmaRx, DmaRxInt, DmaTx, DmaTxInt>
{
    fn select<CsPin: GpioPinMap>(&mut self, chip: &SpiChip<CsPin>) {
        // Clear output pin by writing BR (bit reset) to the bit set/reset register.
        chip.cs.gpio_bsrr_br.set_bit();
    }

    fn deselect<CsPin: GpioPinMap>(&mut self, chip: &SpiChip<CsPin>) {
        // Set output pin by writing BS (bit set) to the bit set/reset register.
        chip.cs.gpio_bsrr_bs.set_bit();
    }
}