mod usart1;
mod usart2;

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f417",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f437",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
mod usart3;

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f417",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f437",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
mod uart4;

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f417",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f437",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
mod uart5;

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f410",
    stm32_mcu = "stm32f411",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
mod usart6;

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f417",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f437",
    stm32_mcu = "stm32f469",
))]
mod uart7;

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f417",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f437",
    stm32_mcu = "stm32f469",
))]
mod uart8;

#[cfg(any(stm32_mcu = "stm32f413",))]
mod uart9;

#[cfg(any(stm32_mcu = "stm32f413",))]
mod uart10;

pub use self::usart1::*;
pub use self::usart2::*;

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f417",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f437",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
pub use self::usart3::*;

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f417",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f437",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
pub use self::uart4::*;

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f417",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f437",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
pub use self::uart5::*;

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f410",
    stm32_mcu = "stm32f411",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
pub use self::usart6::*;

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f417",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f437",
    stm32_mcu = "stm32f469",
))]
pub use self::uart7::*;

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f417",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f437",
    stm32_mcu = "stm32f469",
))]
pub use self::uart8::*;

#[cfg(any(stm32_mcu = "stm32f413",))]
pub use self::uart9::*;

#[cfg(any(stm32_mcu = "stm32f413",))]
pub use self::uart10::*;
