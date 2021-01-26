use core::marker::PhantomData;

use alloc::sync::Arc;
use drone_stm32f4_gpio_drv::{AlternateMode, GpioPin, GpioPinMap, PinAf};

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
pub struct InputCaptureMode<Pin: GpioPinMap, Af: PinAf, PinType: Send, PinPull: Send, Sel: Send> {
    // pub pin: Arc<GpioPin<Pin, AlternateMode<Af>, PinType, PinPull>>,
    pin: PhantomData<Pin>,
    af: PhantomData<Af>,
    typ: PhantomData<PinType>,
    pull: PhantomData<PinPull>,
    sel: PhantomData<Sel>,
}

impl<Pin: GpioPinMap, Af: PinAf, PinType: Send, PinPull: Send, Sel: Send> InputCaptureMode<Pin, Af, PinType, PinPull, Sel> {
    pub fn new(pin: GpioPin<Pin, AlternateMode<Af>, PinType, PinPull>) -> Self {
        Self {
            // pin: Arc::new(pin),
            pin: PhantomData,
            af: PhantomData,
            typ: PhantomData,
            pull: PhantomData,
            sel: PhantomData
        }
    }
}

/// Channel X maps directly to the input for channel X.
pub struct DirectSelection;

/// Channel X maps to its indirect neighbour channel Y.
pub struct IndirectSelection;

/// TRC selection
pub struct TrcSelection;
