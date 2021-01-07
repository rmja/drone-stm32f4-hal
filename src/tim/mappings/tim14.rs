use drone_stm32_map::periph::tim::general::Tim14;
use drone_stm32f4_rcc_drv::clktree::PClk1;

general_tim_setup!(Tim14, PClk1);
