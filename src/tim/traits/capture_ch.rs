use core::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::Stream;

pub struct ChannelCaptureOverflow;

pub trait TimerCaptureCh {
    /// Capture control handler.
    type Control: CaptureControl;

    /// Get a stream of captured timer values that yield for each input capture on the timer channel.
    /// When the underlying ring buffer overflows, new items will be skipped.
    fn saturating_stream(&mut self, capacity: usize) -> CaptureStream<'_, Self::Control, u32>;

    /// Get a stream of captured timer values that yield for each input capture on the timer channel.
    /// When the underlying ring buffer overflows, new items will overwrite existing ones.
    fn overwriting_stream(&mut self, capacity: usize) -> CaptureStream<'_, Self::Control, u32>;

    /// Get a stream of captured timer values that yield for each input capture on the timer channel.
    /// When the underlying ring buffer overflows, new items will be skipped.
    fn try_stream(
        &mut self,
        capacity: usize,
    ) -> CaptureStream<'_, Self::Control, Result<u32, ChannelCaptureOverflow>>;
}

pub trait CaptureControl: Send {
    /// Get the current value of the underlying pin.
    fn get(&self) -> bool;

    /// Stop the capture stream.
    fn stop(&mut self);
}

pub struct CaptureStream<'a, Control: CaptureControl, Item> {
    control: &'a mut Control,
    stream: Pin<Box<dyn Stream<Item = Item> + Send + 'a>>,
}

impl<'a, Control: CaptureControl, Item> CaptureStream<'a, Control, Item> {
    pub fn new(control: &'a mut Control, stream: Pin<Box<dyn Stream<Item = Item> + Send + 'a>>) -> Self {
        Self { control, stream }
    }

    /// Get the current value of the underlying pin.
    pub fn get(&self) -> bool {
        self.stop.get()
    }

    /// Stop the capture stream.
    #[inline]
    pub fn stop(mut self: Pin<&mut Self>) {
        self.stop.stop();
    }
}

impl<Control: CaptureControl, Item> Stream for CaptureStream<'_, Control, Item> {
    type Item = Item;

    #[inline]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.stream.as_mut().poll_next(cx)
    }
}

impl<Control: CaptureControl, Item> Drop for CaptureStream<'_, Control, Item> {
    #[inline]
    fn drop(&mut self) {
        self.stop.stop();
    }
}
