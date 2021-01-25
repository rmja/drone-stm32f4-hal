use core::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::Stream;

pub struct ChannelCaptureOverflow;

pub trait TimerCaptureCh {
    /// Capture stop handler.
    type Stop: CaptureStop;

    fn get(&self) -> bool;

    fn clear_pending(&mut self);

    /// Get a stream of captured timer values that yield for each input capture on the timer channel.
    /// When the underlying ring buffer overflows, new items will be skipped.
    fn saturating_stream(&mut self, capacity: usize) -> CaptureStream<'_, Self::Stop, u32>;

    /// Get a stream of captured timer values that yield for each input capture on the timer channel.
    /// When the underlying ring buffer overflows, new items will overwrite existing ones.
    fn overwriting_stream(&mut self, capacity: usize) -> CaptureStream<'_, Self::Stop, u32>;

    /// Get a stream of captured timer values that yield for each input capture on the timer channel.
    /// When the underlying ring buffer overflows, new items will be skipped.
    fn try_stream(
        &mut self,
        capacity: usize,
    ) -> CaptureStream<'_, Self::Stop, Result<u32, ChannelCaptureOverflow>>;
}

pub trait CaptureStop: Send {
    /// Stop the capture stream.
    fn stop(&mut self);
}

pub struct CaptureStream<'a, Stop: CaptureStop, Item> {
    stop: &'a mut Stop,
    stream: Pin<Box<dyn Stream<Item = Item> + Send + 'a>>,
}

impl<'a, Stop: CaptureStop, Item> CaptureStream<'a, Stop, Item> {
    pub fn new(stop: &'a mut Stop, stream: Pin<Box<dyn Stream<Item = Item> + Send + 'a>>) -> Self {
        Self { stop, stream }
    }

    /// Stop the capture stream.
    #[inline]
    pub fn stop(mut self: Pin<&mut Self>) {
        self.stop.stop();
    }
}

impl<Stop: CaptureStop, Item> Stream for CaptureStream<'_, Stop, Item> {
    type Item = Item;

    #[inline]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.stream.as_mut().poll_next(cx)
    }
}

impl<Stop: CaptureStop, Item> Drop for CaptureStream<'_, Stop, Item> {
    #[inline]
    fn drop(&mut self) {
        self.stop.stop();
    }
}
