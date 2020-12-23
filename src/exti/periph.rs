use drone_core::periph;

periph::singular! {
    /// Extracts SYSCFG register tokens.
    pub macro periph_syscfg;
    /// SYSCFG peripheral.
    pub struct SyscfgPeriph;
    // Path prefix to reach registers.
    drone_stm32_map::reg;
    // Absolute path to the current module.
    crate;

    RCC {
        APB2ENR {
            SYSCFGEN;
        }
    }
}
