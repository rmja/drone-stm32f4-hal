use drone_core::fib::FiberStreamPulse;

pub trait TimerOverflow {
    /// Get a stream of pulses that yield for each timer overflow.
    fn saturating_pulse_stream(&mut self) -> FiberStreamPulse;
}
