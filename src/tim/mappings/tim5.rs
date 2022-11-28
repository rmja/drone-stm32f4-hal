use crate::{
    general_tim_ch, general_tim_channel, general_tim_setup, shared::DontCare, ConfigureTimCh1,
    ConfigureTimCh2, ConfigureTimCh3, ConfigureTimCh4, DirectSelection, GeneralTimChDrv,
    IndirectSelection, TimCh1, TimCh2, TimCh3, TimCh4,
};
use drone_stm32_map::periph::gpio::pin::*;
use drone_stm32_map::periph::tim::general::Tim5;
use drone_stm32f4_gpio_drv::PinAf2;
use drone_stm32f4_rcc_drv::clktree::PClk1;

general_tim_setup!(Tim5, PClk1);

general_tim_ch!(TimCh1; ConfigureTimCh1<Tim5, ..., ChMode2, ChMode3, ChMode4>.ch1; ch2, ch3, ch4 -> GeneralTimChDrv<ChMode, ChMode2, ChMode3, ChMode4> for GeneralTimCfg<DontCare, ChMode2, ChMode3, ChMode4>);
general_tim_ch!(TimCh2; ConfigureTimCh2<Tim5, ..., ChMode1, ChMode3, ChMode4>.ch2; ch1, ch3, ch4 -> GeneralTimChDrv<ChMode1, ChMode, ChMode3, ChMode4> for GeneralTimCfg<ChMode1, DontCare, ChMode3, ChMode4>);
general_tim_ch!(TimCh3; ConfigureTimCh3<Tim5, ..., ChMode1, ChMode2, ChMode4>.ch3; ch1, ch2, ch4 -> GeneralTimChDrv<ChMode1, ChMode2, ChMode, ChMode4> for GeneralTimCfg<ChMode1, ChMode2, DontCare, ChMode4>);
general_tim_ch!(TimCh4; ConfigureTimCh4<Tim5, ..., ChMode1, ChMode2, ChMode3>.ch4; ch1, ch2, ch3 -> GeneralTimChDrv<ChMode1, ChMode2, ChMode3, ChMode> for GeneralTimCfg<ChMode1, ChMode2, ChMode3, DontCare>);

general_tim_channel!(
    TimCh1<Tim5>, GpioA0<PinAf2> -> DirectSelection;
    TimCh1<Tim5>, GpioB6<PinAf2> -> DirectSelection;
    TimCh1<Tim5>, GpioD12<PinAf2> -> DirectSelection;
    TimCh2<Tim5>, GpioA0<PinAf2> -> IndirectSelection;
    TimCh2<Tim5>, GpioB6<PinAf2> -> IndirectSelection;
    TimCh2<Tim5>, GpioD12<PinAf2> -> IndirectSelection;

    TimCh2<Tim5>, GpioA1<PinAf2> -> DirectSelection;
    TimCh2<Tim5>, GpioB7<PinAf2> -> DirectSelection;
    TimCh2<Tim5>, GpioD13<PinAf2> -> DirectSelection;
    TimCh1<Tim5>, GpioA1<PinAf2> -> IndirectSelection;
    TimCh1<Tim5>, GpioB7<PinAf2> -> IndirectSelection;
    TimCh1<Tim5>, GpioD13<PinAf2> -> IndirectSelection;

    TimCh3<Tim5>, GpioA2<PinAf2> -> DirectSelection;
    TimCh3<Tim5>, GpioB8<PinAf2> -> DirectSelection;
    TimCh3<Tim5>, GpioD14<PinAf2> -> DirectSelection;
    TimCh4<Tim5>, GpioA2<PinAf2> -> IndirectSelection;
    TimCh4<Tim5>, GpioB8<PinAf2> -> IndirectSelection;
    TimCh4<Tim5>, GpioD14<PinAf2> -> IndirectSelection;

    TimCh4<Tim5>, GpioA3<PinAf2> -> DirectSelection;
    TimCh4<Tim5>, GpioB9<PinAf2> -> DirectSelection;
    TimCh4<Tim5>, GpioD15<PinAf2> -> DirectSelection;
    TimCh3<Tim5>, GpioA3<PinAf2> -> IndirectSelection;
    TimCh3<Tim5>, GpioB9<PinAf2> -> IndirectSelection;
    TimCh3<Tim5>, GpioD15<PinAf2> -> IndirectSelection;
);

#[cfg(any(
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f412",
    drone_stm32_map = "stm32f413",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f446",
    drone_stm32_map = "stm32f469",
))]
general_tim_channel!(
    TimCh1<Tim5>, GpioH10<PinAf2> -> DirectSelection;
    TimCh2<Tim5>, GpioH10<PinAf2> -> IndirectSelection;

    TimCh2<Tim5>, GpioH11<PinAf2> -> DirectSelection;
    TimCh1<Tim5>, GpioH11<PinAf2> -> IndirectSelection;

    TimCh3<Tim5>, GpioH12<PinAf2> -> DirectSelection;
    TimCh4<Tim5>, GpioH12<PinAf2> -> IndirectSelection;
);

#[cfg(any(
    drone_stm32_map = "stm32f405",
    drone_stm32_map = "stm32f407",
    drone_stm32_map = "stm32f427",
    drone_stm32_map = "stm32f429",
    drone_stm32_map = "stm32f469",
))]
general_tim_channel!(
    TimCh4<Tim5>, GpioI0<PinAf2> -> DirectSelection;
    TimCh3<Tim5>, GpioI0<PinAf2> -> IndirectSelection;
);
