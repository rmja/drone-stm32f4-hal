use drone_core::reg::prelude::*;
use drone_core::token::Token;
use drone_cortexm::map::reg::dwt;

/// Cycle counter stopwatch.
pub struct Stopwatch
{
    last_started: Option<u32>,
    previously_elapsed: Option<u32>
}

impl Stopwatch {
    /// Start a new stopwatch.
    #[inline]
    pub fn start_new() -> Self {
        Self {
            last_started: Some(cyccnt()),
            previously_elapsed: None,
        }
    }
    
    /// Start a previously stopped stopwatch.
    #[inline]
    pub fn start(&mut self) {
        assert!(self.last_started.is_none(), "Not stopped");
        self.last_started = Some(cyccnt());
    }

    /// Stop an already running stopwatch.
    #[inline]
    pub fn stop(&mut self) {
        let now = cyccnt();
        let running_elapsed = now.wrapping_sub(self.last_started.expect("Not running"));
        self.previously_elapsed = Some(self.previously_elapsed.map_or(running_elapsed, |x| x + running_elapsed));
    }

    /// Get the total elapsed time in cpu cycles
    #[inline]
    pub fn elapsed(&self) -> u32 {
        let now = cyccnt();
        let running_elapsed = self.last_started.map_or(0, |x| now.wrapping_sub(x));
        self.previously_elapsed.map_or(running_elapsed, |x| x + running_elapsed)
    }
}

#[inline]
fn cyccnt() -> u32 {
    let cyccnt = unsafe { dwt::Cyccnt::<Urt>::take() };
    cyccnt.load_bits()
}