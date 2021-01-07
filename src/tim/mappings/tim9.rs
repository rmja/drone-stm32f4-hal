use drone_stm32_map::periph::tim::general::Tim9;
use drone_stm32f4_rcc_drv::clktree::PClk2;

general_tim_setup!(Tim9, PClk2);
