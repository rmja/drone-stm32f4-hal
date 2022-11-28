mod usart1;
mod usart2;

#[cfg(any(
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f412",
    drone_stm32_map = "stm32f413",
    drone_stm32_map = "stm32f417",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f437",
    drone_stm32_map = "stm32f446",
    drone_stm32_map = "stm32f469",
))]
mod usart3;

#[cfg(any(
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f413",
    drone_stm32_map = "stm32f417",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f437",
    drone_stm32_map = "stm32f446",
    drone_stm32_map = "stm32f469",
))]
mod uart4;

#[cfg(any(
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f413",
    drone_stm32_map = "stm32f417",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f437",
    drone_stm32_map = "stm32f446",
    drone_stm32_map = "stm32f469",
))]
mod uart5;

#[cfg(any(
    drone_stm32_map = "stm32f401",
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f410",
    drone_stm32_map = "stm32f411",
    drone_stm32_map = "stm32f412",
    drone_stm32_map = "stm32f413",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f446",
    drone_stm32_map = "stm32f469",
))]
mod usart6;

#[cfg(any(
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f417",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f437",
    drone_stm32_map = "stm32f469",
))]
mod uart7;

#[cfg(any(
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f413",
    drone_stm32_map = "stm32f417",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f437",
    drone_stm32_map = "stm32f469",
))]
mod uart8;

#[cfg(any(drone_stm32_map = "stm32f413",))]
mod uart9;

#[cfg(any(drone_stm32_map = "stm32f413",))]
mod uart10;

pub use self::usart1::*;
pub use self::usart2::*;

#[cfg(any(
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f412",
    drone_stm32_map = "stm32f413",
    drone_stm32_map = "stm32f417",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f437",
    drone_stm32_map = "stm32f446",
    drone_stm32_map = "stm32f469",
))]
pub use self::usart3::*;

#[cfg(any(
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f413",
    drone_stm32_map = "stm32f417",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f437",
    drone_stm32_map = "stm32f446",
    drone_stm32_map = "stm32f469",
))]
pub use self::uart4::*;

#[cfg(any(
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f413",
    drone_stm32_map = "stm32f417",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f437",
    drone_stm32_map = "stm32f446",
    drone_stm32_map = "stm32f469",
))]
pub use self::uart5::*;

#[cfg(any(
    drone_stm32_map = "stm32f401",
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f410",
    drone_stm32_map = "stm32f411",
    drone_stm32_map = "stm32f412",
    drone_stm32_map = "stm32f413",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f446",
    drone_stm32_map = "stm32f469",
))]
pub use self::usart6::*;

#[cfg(any(
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f417",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f437",
    drone_stm32_map = "stm32f469",
))]
pub use self::uart7::*;

#[cfg(any(
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f413",
    drone_stm32_map = "stm32f417",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f437",
    drone_stm32_map = "stm32f469",
))]
pub use self::uart8::*;

#[cfg(any(drone_stm32_map = "stm32f413",))]
pub use self::uart9::*;

#[cfg(any(drone_stm32_map = "stm32f413",))]
pub use self::uart10::*;
