use crate::general_tim_setup;
use drone_stm32_map::periph::tim::general::Tim13;
use drone_stm32f4_rcc_drv::clktree::PClk1;

general_tim_setup!(Tim13, PClk1);
