use drone_core::periph;

periph::singular! {
    /// Extracts FMC register tokens.
    pub macro periph_fmc;
    /// FMC peripheral.
    pub struct FmcPeriph;
    // Path prefix to reach registers.
    drone_stm32_map::reg;
    // Absolute path to the current module.
    crate;

    RCC {
        AHB3ENR {
            FMCEN;
        }
    }
    FMC {
        SDCR1;
        SDCR2;
        SDTR1;
        SDTR2;
    }
}