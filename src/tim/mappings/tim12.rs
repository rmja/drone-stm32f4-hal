use crate::general_tim_setup;
use drone_stm32_map::periph::tim::general::Tim12;
use drone_stm32f4_rcc_drv::clktree::PClk1;

general_tim_setup!(Tim12, PClk1);
