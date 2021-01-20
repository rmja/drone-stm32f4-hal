pub trait TimerCounter: Sync {
    /// Get the current counter value.
    fn value(&self) -> u32;
}
