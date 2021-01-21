/// The timer is counting from 0 to its max value.
pub struct DirCountUp;

/// The timer is counting from its max value to 0.
pub struct DirCountDown;

pub trait TimerCounter: Sync {
    /// Get the current counter value.
    fn value(&self) -> u32;
}
