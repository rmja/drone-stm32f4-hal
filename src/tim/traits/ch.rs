use core::marker::PhantomData;

/// Capture/Compare channel 1.
pub struct TimCh1;

/// Capture/Compare channel 2.
pub struct TimCh2;

/// Capture/Compare channel 3.
pub struct TimCh3;

/// Capture/Compare channel 4.
pub struct TimCh4;

pub trait TimChToken {}
impl TimChToken for TimCh1 {}
impl TimChToken for TimCh2 {}
impl TimChToken for TimCh3 {}
impl TimChToken for TimCh4 {}

/// Timer Output Compare mode.
pub struct OutputCompareMode;

/// Timer Input Capture mode.
pub struct InputCaptureMode<Sel: SelectionToken>(PhantomData<Sel>);

pub trait ChModeToken {
    const CC_SEL: u32;
}
impl ChModeToken for OutputCompareMode {
    const CC_SEL: u32 = 0b00;
}
impl<Sel: SelectionToken> ChModeToken for InputCaptureMode<Sel> {
    const CC_SEL: u32 = Sel::CC_SEL;
}

/// Channel X maps directly to the input for channel X.
pub struct DirectSelection;

/// Channel X maps to its indirect neighbour channel Y.
pub struct IndirectSelection;

/// TRC selection
pub struct TrcSelection;

pub trait SelectionToken {
    const CC_SEL: u32;
}
impl SelectionToken for DirectSelection {
    const CC_SEL: u32 = 0b01;
}
impl SelectionToken for IndirectSelection {
    const CC_SEL: u32 = 0b10;
}
impl SelectionToken for TrcSelection {
    const CC_SEL: u32 = 0b11;
}
