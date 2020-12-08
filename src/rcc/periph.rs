use drone_core::periph;

periph::singular! {
    /// Extracts RCC register tokens.
    pub macro periph_rcc;
    /// RCC peripheral.
    pub struct RccPeriph;
    // Path prefix to reach registers.
    drone_stm32_map::reg;
    // Absolute path to the current module.
    crate;

    RCC {
        CR;
        PLLCFGR;
        CFGR;
        CIR;
    }
}

periph::singular! {
    /// Extracts PWR register tokens.
    pub macro periph_pwr;
    /// PWR peripheral.
    pub struct PwrPeriph;
    // Path prefix to reach registers.
    drone_stm32_map::reg;
    // Absolute path to the current module.
    crate;

    RCC {
        APB1ENR {
            PWREN;
        }
    }
    PWR {
        CR;
        CSR;
    }
}

periph::singular! {
    /// Extracts FLASH register tokens.
    pub macro periph_flash;
    /// FLASH peripheral.
    pub struct FlashPeriph;
    // Path prefix to reach registers.
    drone_stm32_map::reg;
    // Absolute path to the current module.
    crate;

    FLASH {
        ACR;
    }
}
