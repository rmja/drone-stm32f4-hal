use crate::master::SpiMasterDrv;
use drone_cortexm::thr::prelude::*;
use drone_stm32_map::periph::{dma::ch::DmaChMap, gpio::pin::GpioPinMap, spi::SpiMap};
use drone_stm32f4_gpio_drv::{GpioPin, OutputMode};

pub struct SpiChip<Pin: GpioPinMap, PinType, PinPull> {
    cs: GpioPin<Pin, OutputMode, PinType, PinPull>,
}

impl<Pin: GpioPinMap, PinType, PinPull> SpiChip<Pin, PinType, PinPull> {
    /// Select the chip by setting the CS pin low.
    pub fn select(&mut self) {
        self.cs.clear();
    }

    /// Deselect the chip by setting the CS pin high.
    pub fn deselect(&mut self) {
        self.cs.set();
    }
}

impl<Pin: GpioPinMap, PinType, PinPull> SpiChip<Pin, PinType, PinPull> {
    /// Initialize a new `SpiChip` as deselected.
    pub fn as_deselected(cs: GpioPin<Pin, OutputMode, PinType, PinPull>) -> Self {
        let mut chip = Self { cs };
        chip.deselect();
        chip
    }
}

pub struct SelectGuard<'a, Pin: GpioPinMap, PinType, PinPull> {
    chip: &'a mut SpiChip<Pin, PinType, PinPull>,
}

impl<Pin: GpioPinMap, PinType, PinPull> Drop
    for SelectGuard<'_, Pin, PinType, PinPull>
{
    fn drop(&mut self) {
        self.chip.deselect();
    }
}

pub trait ChipCtrl {
    /// Select a specific chip and return a guard that deselects the chip when dropped.
    fn select<'guard, Pin: GpioPinMap, PinType, PinPull>(
        &mut self,
        chip: &'guard mut SpiChip<Pin, PinType, PinPull>,
    ) -> SelectGuard<'guard, Pin, PinType, PinPull> {
        chip.select();
        SelectGuard { chip }
    }
}

impl<
        'drv,
        Spi: SpiMap,
        DmaRx: DmaChMap,
        DmaRxInt: IntToken,
        DmaTx: DmaChMap,
        DmaTxInt: IntToken,
    > ChipCtrl for SpiMasterDrv<'drv, Spi, DmaRx, DmaRxInt, DmaTx, DmaTxInt>
{
}
