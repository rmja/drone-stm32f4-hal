use crate::master::SpiMasterDrv;
use drone_cortexm::thr::prelude::*;
use drone_stm32_map::periph::{dma::ch::DmaChMap, gpio::pin::GpioPinMap, spi::SpiMap};
use drone_stm32f4_gpio_drv::{GpioPin, OutputMode, PinPullToken, PinTypeToken};

pub struct SpiChip<Pin: GpioPinMap, PinType: PinTypeToken, PinPull: PinPullToken> {
    cs: GpioPin<Pin, OutputMode, PinType, PinPull>,
}

impl<Pin: GpioPinMap, PinType: PinTypeToken, PinPull: PinPullToken> SpiChip<Pin, PinType, PinPull> {
    pub fn init(cs: GpioPin<Pin, OutputMode, PinType, PinPull>) -> Self {
        // Set the CS pin to high.
        cs.set();
        Self { cs }
    }
}

pub trait ChipCtrl {
    fn select<CsPin: GpioPinMap, PinType: PinTypeToken, PinPull: PinPullToken>(
        &mut self,
        chip: &SpiChip<CsPin, PinType, PinPull>,
    );
    fn deselect<CsPin: GpioPinMap, PinType: PinTypeToken, PinPull: PinPullToken>(
        &mut self,
        chip: &SpiChip<CsPin, PinType, PinPull>,
    );
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
    fn select<CsPin: GpioPinMap, PinType: PinTypeToken, PinPull: PinPullToken>(
        &mut self,
        chip: &SpiChip<CsPin, PinType, PinPull>,
    ) {
        // Clear the CS pin to low.
        chip.cs.clear();
    }

    fn deselect<CsPin: GpioPinMap, PinType: PinTypeToken, PinPull: PinPullToken>(
        &mut self,
        chip: &SpiChip<CsPin, PinType, PinPull>,
    ) {
        // Set the CS pin to high.
        chip.cs.set();
    }
}
