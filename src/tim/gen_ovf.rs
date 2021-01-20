use core::marker::PhantomData;

use alloc::sync::Arc;
use drone_core::{
    fib::{self, FiberStreamPulse},
    reg::prelude::*,
    thr::prelude::*,
    token::Token,
};
use drone_cortexm::thr::IntToken;
use drone_stm32_map::periph::tim::general::{traits::*, GeneralTimMap, GeneralTimPeriph};

use crate::TimerOverflow;

pub struct GeneralTimOvfDrv<Tim: GeneralTimMap, Int: IntToken> {
    tim: PhantomData<Tim>,
    tim_int: Int,
}

impl<Tim: GeneralTimMap, Int: IntToken> GeneralTimOvfDrv<Tim, Int> {
    pub(crate) fn new(_tim: Arc<GeneralTimPeriph<Tim>>, tim_int: Int) -> Self {
        Self {
            tim: PhantomData,
            tim_int,
        }
    }
}

impl<Tim: GeneralTimMap, Int: IntToken> TimerOverflow for GeneralTimOvfDrv<Tim, Int> {
    fn saturating_pulse_stream(&mut self) -> FiberStreamPulse {
        let tim_sr = unsafe { Tim::CTimSr::take() };
        self.tim_int
            .add_saturating_pulse_stream(fib::new_fn(move || {
                if tim_sr.uif().read_bit() {
                    // rc_w0: Clear flag by writing a 0, 1 has no effect.
                    let mut val = unsafe { Tim::STimSr::val_from(u32::MAX) };
                    tim_sr.uif().clear(&mut val);
                    tim_sr.store_val(val);

                    fib::Yielded(Some(1))
                } else {
                    fib::Yielded(None)
                }
            }))
    }
}
