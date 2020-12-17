mod spi1;
mod spi2;
mod spi3;
#[cfg(any(
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
mod spi4;
#[cfg(any(
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
mod spi5;
#[cfg(any(stm32_mcu = "stm32f427", stm32_mcu = "stm32f469",))]
mod spi6;

pub use self::spi1::*;
pub use self::spi2::*;
pub use self::spi3::*;
#[cfg(any(
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
pub use self::spi4::*;
#[cfg(any(
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
pub use self::spi5::*;
#[cfg(any(stm32_mcu = "stm32f427", stm32_mcu = "stm32f469",))]
pub use self::spi6::*;
