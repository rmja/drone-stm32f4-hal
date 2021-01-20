use core::marker::PhantomData;

use drone_core::fib::FiberStreamPulse;
use drone_cortexm::thr::IntToken;
use drone_stm32f4_rcc_drv::clktree::PClkToken;

pub struct DontCare;

pub enum TimFreq {
    Nominal(u32),
    Prescaler(u16),
}

pub struct DirCountUp;
pub struct DirCountDown;

pub trait DirToken {}
impl DirToken for DirCountUp {}
impl DirToken for DirCountDown {}

pub struct DefaultLink;
pub struct MasterLink<MasterTim>(PhantomData<MasterTim>);
pub struct SlaveLink<MasterTim>(PhantomData<MasterTim>);

pub trait LinkToken {}
impl LinkToken for DefaultLink {}
impl<MasterTim> LinkToken for MasterLink<MasterTim> {}
impl<MasterTim> LinkToken for SlaveLink<MasterTim> {}

pub trait TimerLink<
    Tim,
    Int: IntToken,
    Clk: PClkToken,
    Dir: DirToken,
    Ch1Mode,
    Ch2Mode,
    Ch3Mode,
    Ch4Mode,
    MasterTim,
>
{
    type Into;

    /// The counter starts at a rising edge of the trigger TRGI (but it is not reset).
    /// Only the start of the counter is controlled.
    fn into_trigger_slave_of(self, master_link: PhantomData<MasterLink<MasterTim>>) -> Self::Into;
}

pub trait TimerCounter: Sync {
    /// Get the current counter value.
    fn value(&self) -> u32;
}

pub trait TimerOverflow {
    fn saturating_pulse_stream(&mut self) -> FiberStreamPulse;
}

pub struct ChannelCaptureOverflow;

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
