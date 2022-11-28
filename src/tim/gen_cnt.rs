use core::marker::PhantomData;

use alloc::sync::Arc;
use drone_core::reg::prelude::*;
use drone_stm32_map::periph::tim::general::{traits::*, GeneralTimMap};

use crate::{gen::GeneralTimDiverged, TimerCounter};

pub struct GeneralTimCntDrv<Tim: GeneralTimMap, Dir: Send + Sync>(
    Arc<GeneralTimDiverged<Tim>>,
    PhantomData<Dir>,
);

impl<Tim: GeneralTimMap, Dir: Send + Sync> GeneralTimCntDrv<Tim, Dir> {
    /// Create a new timer counter driver.
    pub(crate) fn new(tim: Arc<GeneralTimDiverged<Tim>>, _dir: Dir) -> Self {
        Self(tim, PhantomData)
    }

    pub(crate) fn into<ToDir: Send + Sync>(self) -> GeneralTimCntDrv<Tim, ToDir> {
        GeneralTimCntDrv(self.0, PhantomData)
    }
}

impl<Tim: GeneralTimMap, Dir: Send + Sync> TimerCounter for GeneralTimCntDrv<Tim, Dir> {
    fn value(&self) -> u32 {
        self.0.tim_cnt.cnt().read_bits()
    }
}

impl<Tim: GeneralTimMap, Dir: Send + Sync> Clone for GeneralTimCntDrv<Tim, Dir> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}
