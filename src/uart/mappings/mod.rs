#[cfg(any(stm32_mcu = "stm32f427", stm32_mcu = "stm32f429",))]
mod f427_f429;

#[cfg(any(stm32_mcu = "stm32f427", stm32_mcu = "stm32f429",))]
pub use self::f427_f429::*;
