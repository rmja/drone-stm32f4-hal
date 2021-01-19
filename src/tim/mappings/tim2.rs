use crate::{
    shared::DontCare, ConfigureTimCh1, ConfigureTimCh2, ConfigureTimCh3, ConfigureTimCh4,
    DirectSelection, IndirectSelection, TimCh1, TimCh2, TimCh3, TimCh4, TimChCfg,
};
use drone_stm32_map::periph::gpio::pin::*;
use drone_stm32_map::periph::tim::general::Tim2;
use drone_stm32f4_gpio_drv::PinAf1;
use drone_stm32f4_rcc_drv::clktree::PClk1;

general_tim_setup!(Tim2, PClk1);

general_tim_ch!(TimCh1; ConfigureTimCh1<Tim2, ..., ChMode2, ChMode3, ChMode4>.ch1; ch2, ch3, ch4 -> TimChCfg<ChMode, ChMode2, ChMode3, ChMode4> for GeneralTimCfg<DontCare, ChMode2, ChMode3, ChMode4>);
general_tim_ch!(TimCh2; ConfigureTimCh2<Tim2, ..., ChMode1, ChMode3, ChMode4>.ch2; ch1, ch3, ch4 -> TimChCfg<ChMode1, ChMode, ChMode3, ChMode4> for GeneralTimCfg<ChMode1, DontCare, ChMode3, ChMode4>);
general_tim_ch!(TimCh3; ConfigureTimCh3<Tim2, ..., ChMode1, ChMode2, ChMode4>.ch3; ch1, ch2, ch4 -> TimChCfg<ChMode1, ChMode2, ChMode, ChMode4> for GeneralTimCfg<ChMode1, ChMode2, DontCare, ChMode4>);
general_tim_ch!(TimCh4; ConfigureTimCh4<Tim2, ..., ChMode1, ChMode2, ChMode3>.ch4; ch1, ch2, ch3 -> TimChCfg<ChMode1, ChMode2, ChMode3, ChMode> for GeneralTimCfg<ChMode1, ChMode2, ChMode3, DontCare>);

general_tim_channel!(
    TimCh1<Tim2>, GpioA0<PinAf1> -> DirectSelection;
    TimCh1<Tim2>, GpioA5<PinAf1> -> DirectSelection;
    TimCh1<Tim2>, GpioA15<PinAf1> -> DirectSelection;
    TimCh2<Tim2>, GpioA0<PinAf1> -> IndirectSelection;
    TimCh2<Tim2>, GpioA5<PinAf1> -> IndirectSelection;
    TimCh2<Tim2>, GpioA15<PinAf1> -> IndirectSelection;

    TimCh2<Tim2>, GpioA1<PinAf1> -> DirectSelection;
    TimCh2<Tim2>, GpioB3<PinAf1> -> DirectSelection;
    TimCh1<Tim2>, GpioA1<PinAf1> -> IndirectSelection;
    TimCh1<Tim2>, GpioB3<PinAf1> -> IndirectSelection;

    TimCh3<Tim2>, GpioA2<PinAf1> -> DirectSelection;
    TimCh4<Tim2>, GpioA2<PinAf1> -> IndirectSelection;

    TimCh4<Tim2>, GpioA3<PinAf1> -> DirectSelection;
    TimCh4<Tim2>, GpioB11<PinAf1> -> DirectSelection;
    TimCh3<Tim2>, GpioA3<PinAf1> -> IndirectSelection;
    TimCh3<Tim2>, GpioB11<PinAf1> -> IndirectSelection;
);
