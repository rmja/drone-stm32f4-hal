use core::{
    pin::Pin,
    task::{Context, Poll},
};

use alloc::sync::Arc;
use drone_stm32f4_gpio_drv::{GpioPin, GpioPinMap, prelude::*};
use futures::Stream;

pub struct ChannelCaptureOverflow;

pub enum TimerCapturePolarity {
    /// Non-inverted, rising edge polarity.
    RisingEdge,
    /// Inverted, falling edge polarity.
    FallingEdge,
    /// Non-inverted, both edges polarity.
    BothEdges,
}

pub trait TimerCaptureCh {
    type Stop: CaptureStop;

    /// Get a stream of captured timer values that yield for each input capture on the timer channel.
    /// When the underlying ring buffer overflows, new items will be skipped.
    fn saturating_stream(
        &mut self,
        capacity: usize,
        polarity: TimerCapturePolarity
    ) -> CaptureStream<'_, Self::Stop, u32>;

    /// Get a stream of captured timer values that yield for each input capture on the timer channel.
    /// When the underlying ring buffer overflows, new items will overwrite existing ones.
    fn overwriting_stream(
        &mut self,
        capacity: usize,
        polarity: TimerCapturePolarity
    ) -> CaptureStream<'_, Self::Stop, u32>;

    /// Get a stream of captured timer values that yield for each input capture on the timer channel.
    /// When the underlying ring buffer overflows, new items will be skipped.
    fn try_stream(
        &mut self,
        capacity: usize,
        polarity: TimerCapturePolarity,
    ) -> CaptureStream<'_, Self::Stop, Result<u32, ChannelCaptureOverflow>>;
}
pub trait TimerPinCaptureCh<Pin: GpioPinMap, Af: PinAf, PinType: PinTypeMap, PinPull: PinPullMap>: TimerCaptureCh {
    /// Get a handle for the underlying capture pin.
    fn pin(&self) -> Arc<GpioPin<Pin, AlternateMode<Af>, PinType, PinPull>>;
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
