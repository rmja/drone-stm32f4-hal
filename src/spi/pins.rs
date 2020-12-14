use core::marker::PhantomData;
use drone_stm32_map::periph::gpio::pin::GpioPinMap;
use drone_stm32_map::periph::spi::{SpiMap, SpiPeriph};
use drone_stm32f4_gpio_drv::prelude::*;
use drone_stm32f4_gpio_drv::GpioPin;
use drone_stm32f4_gpio_drv::{pin_ext, pin_impl};

pub struct Undefined;
pub struct Defined;

pub struct SpiPins2<Spi: SpiMap, Sck, Miso, Mosi> {
    spi: PhantomData<Spi>,
    sck: PhantomData<Sck>,
    miso: PhantomData<Miso>,
    mosi: PhantomData<Mosi>,
}

impl<Spi: SpiMap> SpiPins2<Spi, Undefined, Undefined, Undefined> {
    pub fn new() -> SpiPins2<Spi, Undefined, Undefined, Undefined> {
        SpiPins2::default()
    }
}

impl<Spi: SpiMap, Sck, Miso, Mosi> Default for SpiPins2<Spi, Sck, Miso, Mosi> {
    fn default() -> Self {
        Self {
            spi: PhantomData,
            sck: PhantomData,
            miso: PhantomData,
            mosi: PhantomData,
        }
    }
}

pub fn spi_pins_consume<Spi: SpiMap>(
    spi: SpiPeriph<Spi>,
    pins: SpiPins2<Spi, Defined, Defined, Defined>,
) {
}

pin_ext!(SckPinExt<Spi: SpiMap, ..., Sck, Miso, Mosi>.sck -> SpiPins2<Spi, Defined, Miso, Mosi>);
pin_ext!(MisoPinExt<Spi: SpiMap, ..., Sck, Miso, Mosi>.miso -> SpiPins2<Spi, Sck, Defined, Mosi>);
pin_ext!(MosiPinExt<Spi: SpiMap, ..., Sck, Miso, Mosi>.mosi -> SpiPins2<Spi, Sck, Miso, Defined>);

// The mappings from the SPI peripheral to the GPIOs are in the chips datasheet and not the technical manual.
pin_impl!(SckPinExt for SpiPins2<drone_stm32_map::periph::spi::Spi1, ...>.sck, GpioA5, AlternateMode<PinAf5>; Undefined, Miso, Mosi => Defined, Miso, Mosi);
pin_impl!(SckPinExt for SpiPins2<drone_stm32_map::periph::spi::Spi1, ...>.sck, GpioB3, AlternateMode<PinAf5>; Undefined, Miso, Mosi => Defined, Miso, Mosi);
pin_impl!(MisoPinExt for SpiPins2<drone_stm32_map::periph::spi::Spi1, ...>.miso, GpioA6, AlternateMode<PinAf5>; Sck, Undefined, Mosi => Sck, Defined, Mosi);
pin_impl!(MisoPinExt for SpiPins2<drone_stm32_map::periph::spi::Spi1, ...>.miso, GpioB4, AlternateMode<PinAf5>; Sck, Undefined, Mosi => Sck, Defined, Mosi);
pin_impl!(MosiPinExt for SpiPins2<drone_stm32_map::periph::spi::Spi1, ...>.mosi, GpioA7, AlternateMode<PinAf5>; Sck, Miso, Undefined => Sck, Miso, Defined);
pin_impl!(MosiPinExt for SpiPins2<drone_stm32_map::periph::spi::Spi1, ...>.mosi, GpioB5, AlternateMode<PinAf5>; Sck, Miso, Undefined => Sck, Miso, Defined);
