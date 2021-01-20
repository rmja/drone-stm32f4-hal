use core::marker::PhantomData;
use drone_stm32_map::periph::spi::SpiMap;
use drone_stm32f4_gpio_drv::pin_ext;

pub struct Defined;
pub struct Undefined;

pub struct SpiPins<Spi: SpiMap, Sck, Miso, Mosi> {
    spi: PhantomData<Spi>,
    sck: PhantomData<Sck>,
    miso: PhantomData<Miso>,
    mosi: PhantomData<Mosi>,
}

impl<Spi: SpiMap, Sck, Miso, Mosi> SpiPins<Spi, Sck, Miso, Mosi> {
    pub fn new() -> SpiPins<Spi, Sck, Miso, Mosi> {
        Self {
            spi: PhantomData,
            sck: PhantomData,
            miso: PhantomData,
            mosi: PhantomData,
        }
    }
}

impl<Spi: SpiMap> Default for SpiPins<Spi, Undefined, Undefined, Undefined> {
    fn default() -> Self {
        Self::new()
    }
}

pin_ext!(SckPinExt<Spi: SpiMap, ..., Sck, Miso, Mosi>.sck -> SpiPins<Spi, Defined, Miso, Mosi>);
pin_ext!(MisoPinExt<Spi: SpiMap, ..., Sck, Miso, Mosi>.miso -> SpiPins<Spi, Sck, Defined, Mosi>);
pin_ext!(MosiPinExt<Spi: SpiMap, ..., Sck, Miso, Mosi>.mosi -> SpiPins<Spi, Sck, Miso, Defined>);
