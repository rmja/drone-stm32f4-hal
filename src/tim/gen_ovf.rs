use core::num::NonZeroUsize;

use crate::{gen::GeneralTimDiverged, traits::*};
use alloc::sync::Arc;
use drone_core::{fib, reg::prelude::*};
use drone_cortexm::thr::prelude::*;
use drone_stm32_map::periph::tim::general::{traits::*, GeneralTimMap};

pub struct GeneralTimOvfDrv<Tim: GeneralTimMap, Int: IntToken> {
    tim: Arc<GeneralTimDiverged<Tim>>,
    tim_int: Int,
}

impl<Tim: GeneralTimMap, Int: IntToken> GeneralTimOvfDrv<Tim, Int> {
    pub(crate) fn new(tim: Arc<GeneralTimDiverged<Tim>>, tim_int: Int) -> Self {
        Self { tim, tim_int }
    }
}

impl<Tim: GeneralTimMap, Int: IntToken> TimerOverflow for GeneralTimOvfDrv<Tim, Int> {
    type Stop = Self;

    fn int_enable(&self) {
        self.tim.tim_dier.uie().set_bit();
    }

    fn int_disable(&self) {
        self.tim.tim_dier.uie().clear_bit();
    }

    fn is_pending(&self) -> bool {
        self.tim.tim_sr.uif().read_bit()
    }

    fn clear_pending(&self) {
        // rc_w0: Clear flag by writing a 0, 1 has no effect.
        let mut val = unsafe { Tim::CTimSr::val_from(u32::MAX) };
        self.tim.tim_sr.uif().clear(&mut val);
        self.tim.tim_sr.store_val(val);
    }

    fn saturating_pulse_stream(&mut self) -> OverflowStream<'_, Self::Stop, NonZeroUsize> {
        let tim_sr = self.tim.tim_sr;
        let stream = Box::pin(
            self.tim_int
                .add_saturating_pulse_stream(fib::new_fn(move || {
                    if tim_sr.uif().read_bit() {
                        // rc_w0: Clear flag by writing a 0, 1 has no effect.
                        let mut val = unsafe { Tim::CTimSr::val_from(u32::MAX) };
                        tim_sr.uif().clear(&mut val);
                        tim_sr.store_val(val);

                        fib::Yielded(Some(1))
                    } else {
                        fib::Yielded(None)
                    }
                })),
        );
        self.tim.tim_dier.uie().set_bit(); // Enable update interrupt
        OverflowStream::new(self, stream)
    }
}

impl<Tim: GeneralTimMap, Int: IntToken> OverflowStop for GeneralTimOvfDrv<Tim, Int> {
    fn stop(&mut self) {
        self.tim.tim_dier.uie().clear_bit(); // Disable update interrupt
    }
}
