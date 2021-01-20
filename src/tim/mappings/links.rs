use crate::{
    gen::slave_of, DefaultLink, DirToken, GeneralTimCfg, MasterLink, SlaveLink, TimerLink,
};
use core::marker::PhantomData;
use drone_cortexm::thr::IntToken;
use drone_stm32_map::periph::tim::advanced::*;
use drone_stm32_map::periph::tim::general::*;
use drone_stm32f4_rcc_drv::clktree::PClkToken;

macro_rules! timer_link {
    ($type_type:ident<$slave_tim:ident>; $itr0_tim:ident, $itr1_tim:ident, $itr2_tim:ident, $itr3_tim:ident) => {
        impl<Int: IntToken, Clk: PClkToken, Dir: DirToken, Ch1Mode, Ch2Mode, Ch3Mode, Ch4Mode>
            TimerLink<$slave_tim, Int, Clk, Dir, Ch1Mode, Ch2Mode, Ch3Mode, Ch4Mode, $itr0_tim>
            for $type_type<
                $slave_tim,
                Int,
                Clk,
                Dir,
                DefaultLink,
                Ch1Mode,
                Ch2Mode,
                Ch3Mode,
                Ch4Mode,
            >
        {
            type Into = GeneralTimCfg<
                $slave_tim,
                Int,
                Clk,
                Dir,
                SlaveLink<$itr0_tim>,
                Ch1Mode,
                Ch2Mode,
                Ch3Mode,
                Ch4Mode,
            >;

            fn into_trigger_slave_of(
                self,
                _master_link: PhantomData<MasterLink<$itr0_tim>>,
            ) -> Self::Into {
                slave_of(&self.tim, 0b110, 0); // Trigger mode
                self.into()
            }
        }
        impl<Int: IntToken, Clk: PClkToken, Dir: DirToken, Ch1Mode, Ch2Mode, Ch3Mode, Ch4Mode>
            TimerLink<$slave_tim, Int, Clk, Dir, Ch1Mode, Ch2Mode, Ch3Mode, Ch4Mode, $itr1_tim>
            for $type_type<
                $slave_tim,
                Int,
                Clk,
                Dir,
                DefaultLink,
                Ch1Mode,
                Ch2Mode,
                Ch3Mode,
                Ch4Mode,
            >
        {
            type Into = GeneralTimCfg<
                $slave_tim,
                Int,
                Clk,
                Dir,
                SlaveLink<$itr0_tim>,
                Ch1Mode,
                Ch2Mode,
                Ch3Mode,
                Ch4Mode,
            >;

            fn into_trigger_slave_of(
                self,
                _master_link: PhantomData<MasterLink<$itr1_tim>>,
            ) -> Self::Into {
                slave_of(&self.tim, 0b110, 1); // Trigger mode
                self.into()
            }
        }
        impl<Int: IntToken, Clk: PClkToken, Dir: DirToken, Ch1Mode, Ch2Mode, Ch3Mode, Ch4Mode>
            TimerLink<$slave_tim, Int, Clk, Dir, Ch1Mode, Ch2Mode, Ch3Mode, Ch4Mode, $itr2_tim>
            for $type_type<
                $slave_tim,
                Int,
                Clk,
                Dir,
                DefaultLink,
                Ch1Mode,
                Ch2Mode,
                Ch3Mode,
                Ch4Mode,
            >
        {
            type Into = GeneralTimCfg<
                $slave_tim,
                Int,
                Clk,
                Dir,
                SlaveLink<$itr0_tim>,
                Ch1Mode,
                Ch2Mode,
                Ch3Mode,
                Ch4Mode,
            >;

            fn into_trigger_slave_of(
                self,
                _master_link: PhantomData<MasterLink<$itr2_tim>>,
            ) -> Self::Into {
                slave_of(&self.tim, 0b110, 2); // Trigger mode
                self.into()
            }
        }
        impl<Int: IntToken, Clk: PClkToken, Dir: DirToken, Ch1Mode, Ch2Mode, Ch3Mode, Ch4Mode>
            TimerLink<$slave_tim, Int, Clk, Dir, Ch1Mode, Ch2Mode, Ch3Mode, Ch4Mode, $itr3_tim>
            for $type_type<
                $slave_tim,
                Int,
                Clk,
                Dir,
                DefaultLink,
                Ch1Mode,
                Ch2Mode,
                Ch3Mode,
                Ch4Mode,
            >
        {
            type Into = GeneralTimCfg<
                $slave_tim,
                Int,
                Clk,
                Dir,
                SlaveLink<$itr0_tim>,
                Ch1Mode,
                Ch2Mode,
                Ch3Mode,
                Ch4Mode,
            >;

            fn into_trigger_slave_of(
                self,
                _master_link: PhantomData<MasterLink<$itr3_tim>>,
            ) -> Self::Into {
                slave_of(&self.tim, 0b110, 3); // Trigger mode
                self.into()
            }
        }
    };
}

// Table 94. TIMx internal trigger connection
// timer_link!(Tim1; Tim5, Tim2, Tim3, Tim4);
// timer_link!(Tim8; Tim1, Tim2, Tim4, Tim5);

// Table 98. TIMx internal trigger connection
timer_link!(GeneralTimCfg<Tim2>; Tim1, Tim8, Tim3, Tim4);
timer_link!(GeneralTimCfg<Tim3>; Tim1, Tim2, Tim5, Tim4);
timer_link!(GeneralTimCfg<Tim4>; Tim1, Tim2, Tim3, Tim8);
timer_link!(GeneralTimCfg<Tim5>; Tim2, Tim3, Tim4, Tim8);

// Table 101. TIMx internal trigger connection
// timer_link!(Tim9; Tim2, Tim3, Tim10, Tim11);
// timer_link!(Tim12; Tim4, Tim5, Tim13, Tim14);
