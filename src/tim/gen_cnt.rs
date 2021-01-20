use alloc::sync::Arc;
use drone_core::reg::prelude::*;
use drone_stm32_map::periph::tim::general::{traits::*, GeneralTimMap, GeneralTimPeriph};

use crate::TimerCounter;

pub struct GeneralTimCntDrv<Tim: GeneralTimMap>(Arc<GeneralTimPeriph<Tim>>);

impl<Tim: GeneralTimMap> GeneralTimCntDrv<Tim> {
    pub(crate) fn new(tim: Arc<GeneralTimPeriph<Tim>>) -> Self {
        Self(tim)
    }
}

impl<Tim: GeneralTimMap> TimerCounter for GeneralTimCntDrv<Tim> {
    fn value(&self) -> u32 {
        self.0.tim_cnt.cnt().read_bits() as u32
    }
}
