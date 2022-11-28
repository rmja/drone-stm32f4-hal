use crate::master::SpiMasterDrv;
use drone_cortexm::thr::prelude::*;
use drone_stm32_map::periph::{dma::ch::DmaChMap, gpio::pin::GpioPinMap, spi::SpiMap};
use drone_stm32f4_gpio_drv::{prelude::*, GpioPin};

pub struct SpiChip<Pin: GpioPinMap, PinType: PinTypeMap, PinPull: PinPullMap> {
    cs: GpioPin<Pin, OutputMode, PinType, PinPull>,
}

impl<Pin: GpioPinMap, PinType: PinTypeMap, PinPull: PinPullMap> SpiChip<Pin, PinType, PinPull> {
    /// Select the chip by setting the CS pin low.
    #[inline]
    pub fn select(&mut self) {
        self.cs.clear();
    }

    /// Deselect the chip by setting the CS pin high.
    #[inline]
    pub fn deselect(&mut self) {
        self.cs.set();
    }
}

impl<Pin: GpioPinMap, PinType: PinTypeMap> SpiChip<Pin, PinType, PullUp> {
    /// Initialize a new `SpiChip` as deselected.
    pub fn new_deselected(cs: GpioPin<Pin, OutputMode, PinType, PullUp>) -> Self {
        let mut chip = Self { cs };
        chip.deselect();
        chip
    }
}

pub struct SelectGuard<'a, Pin: GpioPinMap, PinType: PinTypeMap, PinPull: PinPullMap> {
    chip: &'a mut SpiChip<Pin, PinType, PinPull>,
}

impl<Pin: GpioPinMap, PinType: PinTypeMap, PinPull: PinPullMap> Drop
    for SelectGuard<'_, Pin, PinType, PinPull>
{
    #[inline]
    fn drop(&mut self) {
        self.chip.deselect();
    }
}

pub trait ChipCtrl {
    /// Select a specific chip and return a guard that deselects the chip when dropped.
    #[inline]
    fn select<'guard, Pin: GpioPinMap, PinType: PinTypeMap, PinPull: PinPullMap>(
        &mut self,
        chip: &'guard mut SpiChip<Pin, PinType, PinPull>,
    ) -> SelectGuard<'guard, Pin, PinType, PinPull> {
        chip.select();
        SelectGuard { chip }
    }
}

impl<Spi: SpiMap, DmaRx: DmaChMap, DmaRxInt: IntToken, DmaTx: DmaChMap, DmaTxInt: IntToken> ChipCtrl
    for SpiMasterDrv<Spi, DmaRx, DmaRxInt, DmaTx, DmaTxInt>
{
}
