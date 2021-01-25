use core::marker::PhantomData;

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
pub struct InputCaptureMode<Pin: GpioPinMap, Af: PinAf, PinType, PinPull, Sel> {
    pub pin: GpioPin<Pin, AlternateMode<Af>, PinType, PinPull>,
    pub(crate) sel: PhantomData<Sel>,
}

/// Channel X maps directly to the input for channel X.
pub struct DirectSelection;

/// Channel X maps to its indirect neighbour channel Y.
pub struct IndirectSelection;

/// TRC selection
pub struct TrcSelection;
