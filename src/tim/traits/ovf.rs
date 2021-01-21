use core::{num::NonZeroUsize, pin::Pin, task::{Context, Poll}};
use futures::Stream;

pub trait TimerOverflow {
    type Stop: OverflowStop;

    fn int_enable(&self);

    fn int_disable(&self);

    fn is_pending(&self) -> bool;

    fn clear_pending(&self);

    /// Get a stream of pulses that yield for each timer overflow.
    fn saturating_pulse_stream(&mut self) -> OverflowStream<'_, Self::Stop, NonZeroUsize>;
}

pub trait OverflowStop: Send {
    /// Stop the overflow stream.
    fn stop(&mut self);
}

pub struct OverflowStream<'a, Stop: OverflowStop, Item> {
    stop: &'a mut Stop,
    stream: Pin<Box<dyn Stream<Item = Item> + Send + 'a>>,
}

impl<'a, Stop: OverflowStop, Item> OverflowStream<'a, Stop, Item> {
    pub fn new(stop: &'a mut Stop, stream: Pin<Box<dyn Stream<Item = Item> + Send + 'a>>) -> Self {
        Self { stop, stream }
    }

    /// Stop the overflow stream.
    pub fn stop(mut self: Pin<&mut Self>) {
        self.stop.stop();
    }
}

impl<Stop: OverflowStop, Item> Stream for OverflowStream<'_, Stop, Item> {
    type Item = Item;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.stream.as_mut().poll_next(cx)
    }
}

impl<Stop: OverflowStop, Item> Drop for OverflowStream<'_, Stop, Item> {
    fn drop(&mut self) {
        self.stop.stop();
    }
}