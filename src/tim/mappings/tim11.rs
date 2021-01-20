use crate::general_tim_setup;
use drone_stm32_map::periph::tim::general::Tim11;
use drone_stm32f4_rcc_drv::clktree::PClk2;

general_tim_setup!(Tim11, PClk2);
