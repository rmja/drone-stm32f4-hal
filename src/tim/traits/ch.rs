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
pub struct InputCaptureMode<Pin: GpioPinMap, Af: PinAf, PinType: Send + Sync, PinPull: Send + Sync, Pol: Send + Sync, Sel: Send + Sync> {
    pub pin: Arc<GpioPin<Pin, AlternateMode<Af>, PinType, PinPull>>,
    pol: PhantomData<Pol>,
    sel: PhantomData<Sel>,
}

impl<Pin: GpioPinMap, Af: PinAf, PinType: Send + Sync, PinPull: Send + Sync, Pol: Send + Sync, Sel: Send + Sync> InputCaptureMode<Pin, Af, PinType, PinPull, Pol, Sel> {
    pub fn new(pin: GpioPin<Pin, AlternateMode<Af>, PinType, PinPull>) -> Self {
        Self {
            pin: Arc::new(pin),
            pol: PhantomData,
            sel: PhantomData,
        }
    }

    pub(crate) fn into<ToPol: Send + Sync, ToSel: Send + Sync>(self) -> InputCaptureMode<Pin, Af, PinType, PinPull, ToPol, ToSel> {
        InputCaptureMode {
            pin: self.pin,
            pol: PhantomData,
            sel: PhantomData,
        }
    }
}

/// Rising edge polarity.
pub struct RisingEdge;

/// Falling edge polarity.
pub struct FallingEdge;

/// Both edges polarity.
pub struct BothEdges;


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