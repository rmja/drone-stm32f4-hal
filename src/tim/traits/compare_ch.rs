use core::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::Future;

pub trait TimerCompareCh {
    type Stop: TimerCompareStop;

    /// Returns a future that resolves when the timer counter is equal to `compare`.
    /// Note that compare is not a duration but an absolute timestamp.
    /// The returned future is resolved immediately if `soon` and
    /// the compare value has already passed with at most `PERIOD/2` ticks.
    fn next(&mut self, compare: u32, soon: bool) -> TimerCompareNext<'_, Self::Stop>;
}

pub trait TimerCompareStop: Send {
    /// Stop the timer.
    fn stop(&mut self);
}

pub struct TimerCompareNext<'a, T: TimerCompareStop> {
    stop: &'a mut T,
    future: Pin<Box<dyn Future<Output = ()> + Send + 'a>>,
}

impl<'a, T: TimerCompareStop> TimerCompareNext<'a, T> {
    pub fn new(stop: &'a mut T, future: Pin<Box<dyn Future<Output = ()> + Send + 'a>>) -> Self {
        Self { stop, future }
    }
}

impl<'a, T: TimerCompareStop> Future for TimerCompareNext<'a, T> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        self.future.as_mut().poll(cx)
    }
}

impl<'a, T: TimerCompareStop> Drop for TimerCompareNext<'a, T> {
    fn drop(&mut self) {
        self.stop.stop();
    }
}
