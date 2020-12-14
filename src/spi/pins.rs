use core::marker::PhantomData;
use drone_stm32_map::periph::gpio::pin::GpioPinMap;
use drone_stm32_map::periph::spi::{SpiMap, SpiPeriph};
use drone_stm32f4_gpio_drv::prelude::*;
use drone_stm32f4_gpio_drv::GpioPin;
use drone_stm32f4_gpio_drv::{pin_ext, pin_impl};

pub struct Defined;
pub struct Undefined;

pub struct SpiPins<Spi: SpiMap, Sck, Miso, Mosi> {
    spi: PhantomData<Spi>,
    sck: PhantomData<Sck>,
    miso: PhantomData<Miso>,
    mosi: PhantomData<Mosi>,
}

impl<Spi: SpiMap> SpiPins<Spi, Undefined, Undefined, Undefined> {
    pub fn new() -> SpiPins<Spi, Undefined, Undefined, Undefined> {
        SpiPins::default()
    }
}

impl<Spi: SpiMap, Sck, Miso, Mosi> Default for SpiPins<Spi, Sck, Miso, Mosi> {
    fn default() -> Self {
        Self {
            spi: PhantomData,
            sck: PhantomData,
            miso: PhantomData,
            mosi: PhantomData,
        }
    }
}

pin_ext!(SckPinExt<Spi: SpiMap, ..., Sck, Miso, Mosi>.sck -> SpiPins<Spi, Defined, Miso, Mosi>);
pin_ext!(MisoPinExt<Spi: SpiMap, ..., Sck, Miso, Mosi>.miso -> SpiPins<Spi, Sck, Defined, Mosi>);
pin_ext!(MosiPinExt<Spi: SpiMap, ..., Sck, Miso, Mosi>.mosi -> SpiPins<Spi, Sck, Miso, Defined>);

// The mappings from the SPI peripheral to the GPIOs are in the chips datasheet and not the technical manual.
pin_impl!(SckPinExt for SpiPins<drone_stm32_map::periph::spi::Spi1, ...>.sck, GpioA5, AlternateMode<PinAf5>; Undefined, Miso, Mosi => Defined, Miso, Mosi);
pin_impl!(SckPinExt for SpiPins<drone_stm32_map::periph::spi::Spi1, ...>.sck, GpioB3, AlternateMode<PinAf5>; Undefined, Miso, Mosi => Defined, Miso, Mosi);
pin_impl!(MisoPinExt for SpiPins<drone_stm32_map::periph::spi::Spi1, ...>.miso, GpioA6, AlternateMode<PinAf5>; Sck, Undefined, Mosi => Sck, Defined, Mosi);
pin_impl!(MisoPinExt for SpiPins<drone_stm32_map::periph::spi::Spi1, ...>.miso, GpioB4, AlternateMode<PinAf5>; Sck, Undefined, Mosi => Sck, Defined, Mosi);
pin_impl!(MosiPinExt for SpiPins<drone_stm32_map::periph::spi::Spi1, ...>.mosi, GpioA7, AlternateMode<PinAf5>; Sck, Miso, Undefined => Sck, Miso, Defined);
pin_impl!(MosiPinExt for SpiPins<drone_stm32_map::periph::spi::Spi1, ...>.mosi, GpioB5, AlternateMode<PinAf5>; Sck, Miso, Undefined => Sck, Miso, Defined);

// SPI2
// - CLK:  B10, B13, D3, I1
// - MISO: B14, C2, I2
// - MOSI: B15, C3, I3
