use core::marker::PhantomData;

use drone_cortexm::thr::IntToken;
use drone_stm32f4_rcc_drv::clktree::PClkToken;

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
    Dir,
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
