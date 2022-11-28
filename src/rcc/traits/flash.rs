pub trait HClkExt {
    /// Get the number of flash access wait states for a given voltage range.
    fn get_wait_states(&self, voltage: VoltageRange) -> u32;
}

#[derive(Copy, Clone)]
pub enum VoltageRange {
    #[doc = "2.7V-3.6V"]
    HighVoltage,
    #[doc = "2.4V-2.7V"]
    MediumVoltage,
    #[doc = "2.1V-2.4V"]
    LowVoltage,
    #[doc = "1.8V-2.1V"]
    UltraLowVoltage,
}
