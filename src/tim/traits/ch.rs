use core::marker::PhantomData;

use alloc::sync::Arc;
use drone_stm32f4_gpio_drv::{prelude::*, GpioPin, GpioPinMap};

/// Capture/Compare channel 1.
pub struct TimCh1;

/// Capture/Compare channel 2.
pub struct TimCh2;

/// Capture/Compare channel 3.
pub struct TimCh3;

/// Capture/Compare channel 4.
pub struct TimCh4;

/// Timer Output Compare mode.
pub struct OutputCompareMode;

/// Timer Input Capture mode.
pub struct InputCaptureMode<
    Pin: GpioPinMap,
    Af: PinAf,
    PinType: PinTypeMap,
    PinPull: PinPullMap,
    Sel: Send + Sync + 'static,
> {
    pub pin: Arc<GpioPin<Pin, AlternateMode<Af>, PinType, PinPull>>,
    sel: PhantomData<Sel>,
}

impl<
        Pin: GpioPinMap,
        Af: PinAf,
        PinType: PinTypeMap,
        PinPull: PinPullMap,
        Sel: Send + Sync + 'static,
    > InputCaptureMode<Pin, Af, PinType, PinPull, Sel>
{
    pub fn new(pin: GpioPin<Pin, AlternateMode<Af>, PinType, PinPull>) -> Self {
        Self {
            pin: Arc::new(pin),
            sel: PhantomData,
        }
    }
}

/// Channel X maps directly to the input for channel X.
pub struct DirectSelection;

/// Channel X maps to its indirect neighbour channel Y.
pub struct IndirectSelection;

/// TRC selection
pub struct TrcSelection;

pub trait InputSelection: Send + Sync + 'static {
    const CC_SEL: u32;
}
impl InputSelection for DirectSelection {
    const CC_SEL: u32 = 0b01;
}
impl InputSelection for IndirectSelection {
    const CC_SEL: u32 = 0b10;
}
impl InputSelection for TrcSelection {
    const CC_SEL: u32 = 0b11;
}
