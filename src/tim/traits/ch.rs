use core::marker::PhantomData;

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
pub struct InputCaptureMode<Sel>(PhantomData<Sel>);

/// Channel X maps directly to the input for channel X.
pub struct DirectSelection;

/// Channel X maps to its indirect neighbour channel Y.
pub struct IndirectSelection;

/// TRC selection
pub struct TrcSelection;