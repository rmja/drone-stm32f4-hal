use crate::pin_init;
use drone_stm32_map::periph::gpio::{head::*, pin::*};

pin_init!(
    GpioAHead, GpioA0;
    GpioAHead, GpioA1;
    GpioAHead, GpioA2;
    GpioAHead, GpioA3;
    GpioAHead, GpioA4;
    GpioAHead, GpioA5;
    GpioAHead, GpioA6;
    GpioAHead, GpioA7;
    GpioAHead, GpioA8;
    GpioAHead, GpioA9;
    GpioAHead, GpioA10;
    GpioAHead, GpioA11;
    GpioAHead, GpioA12;
    GpioAHead, GpioA13;
    GpioAHead, GpioA14;
    GpioAHead, GpioA15;
);

pin_init!(
    GpioBHead, GpioB0;
    GpioBHead, GpioB1;
    GpioBHead, GpioB2;
    GpioBHead, GpioB3;
    GpioBHead, GpioB4;
    GpioBHead, GpioB5;
    GpioBHead, GpioB6;
    GpioBHead, GpioB7;
    GpioBHead, GpioB8;
    GpioBHead, GpioB9;
    GpioBHead, GpioB10;
    GpioBHead, GpioB11;
    GpioBHead, GpioB12;
    GpioBHead, GpioB13;
    GpioBHead, GpioB14;
    GpioBHead, GpioB15;
);

pin_init!(
    GpioCHead, GpioC0;
    GpioCHead, GpioC1;
    GpioCHead, GpioC2;
    GpioCHead, GpioC3;
    GpioCHead, GpioC4;
    GpioCHead, GpioC5;
    GpioCHead, GpioC6;
    GpioCHead, GpioC7;
    GpioCHead, GpioC8;
    GpioCHead, GpioC9;
    GpioCHead, GpioC10;
    GpioCHead, GpioC11;
    GpioCHead, GpioC12;
    GpioCHead, GpioC13;
    GpioCHead, GpioC14;
    GpioCHead, GpioC15;
);

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f411",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
pin_init!(
    GpioDHead, GpioD0;
    GpioDHead, GpioD1;
    GpioDHead, GpioD2;
    GpioDHead, GpioD3;
    GpioDHead, GpioD4;
    GpioDHead, GpioD5;
    GpioDHead, GpioD6;
    GpioDHead, GpioD7;
    GpioDHead, GpioD8;
    GpioDHead, GpioD9;
    GpioDHead, GpioD10;
    GpioDHead, GpioD11;
    GpioDHead, GpioD12;
    GpioDHead, GpioD13;
    GpioDHead, GpioD14;
    GpioDHead, GpioD15;
);

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f411",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
pin_init!(
    GpioEHead, GpioE0;
    GpioEHead, GpioE1;
    GpioEHead, GpioE2;
    GpioEHead, GpioE3;
    GpioEHead, GpioE4;
    GpioEHead, GpioE5;
    GpioEHead, GpioE6;
    GpioEHead, GpioE7;
    GpioEHead, GpioE8;
    GpioEHead, GpioE9;
    GpioEHead, GpioE10;
    GpioEHead, GpioE11;
    GpioEHead, GpioE12;
    GpioEHead, GpioE13;
    GpioEHead, GpioE14;
    GpioEHead, GpioE15;
);

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
pin_init!(
    GpioFHead, GpioF0;
    GpioFHead, GpioF1;
    GpioFHead, GpioF2;
    GpioFHead, GpioF3;
    GpioFHead, GpioF4;
    GpioFHead, GpioF5;
    GpioFHead, GpioF6;
    GpioFHead, GpioF7;
    GpioFHead, GpioF8;
    GpioFHead, GpioF9;
    GpioFHead, GpioF10;
    GpioFHead, GpioF11;
    GpioFHead, GpioF12;
    GpioFHead, GpioF13;
    GpioFHead, GpioF14;
    GpioFHead, GpioF15;
);

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
pin_init!(
    GpioGHead, GpioG0;
    GpioGHead, GpioG1;
    GpioGHead, GpioG2;
    GpioGHead, GpioG3;
    GpioGHead, GpioG4;
    GpioGHead, GpioG5;
    GpioGHead, GpioG6;
    GpioGHead, GpioG7;
    GpioGHead, GpioG8;
    GpioGHead, GpioG9;
    GpioGHead, GpioG10;
    GpioGHead, GpioG11;
    GpioGHead, GpioG12;
    GpioGHead, GpioG13;
    GpioGHead, GpioG14;
    GpioGHead, GpioG15;
);

pin_init!(
    GpioHHead, GpioH0;
    GpioHHead, GpioH1;
    GpioHHead, GpioH2;
    GpioHHead, GpioH3;
    GpioHHead, GpioH4;
    GpioHHead, GpioH5;
    GpioHHead, GpioH6;
    GpioHHead, GpioH7;
    GpioHHead, GpioH8;
    GpioHHead, GpioH9;
    GpioHHead, GpioH10;
    GpioHHead, GpioH11;
    GpioHHead, GpioH12;
    GpioHHead, GpioH13;
    GpioHHead, GpioH14;
    GpioHHead, GpioH15;
);

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
pin_init!(
    GpioIHead, GpioI0;
    GpioIHead, GpioI1;
    GpioIHead, GpioI2;
    GpioIHead, GpioI3;
    GpioIHead, GpioI4;
    GpioIHead, GpioI5;
    GpioIHead, GpioI6;
    GpioIHead, GpioI7;
    GpioIHead, GpioI8;
    GpioIHead, GpioI9;
    GpioIHead, GpioI10;
    GpioIHead, GpioI11;
    GpioIHead, GpioI12;
    GpioIHead, GpioI13;
    GpioIHead, GpioI14;
    GpioIHead, GpioI15;
);

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
pin_init!(
    GpioJHead, GpioJ0;
    GpioJHead, GpioJ1;
    GpioJHead, GpioJ2;
    GpioJHead, GpioJ3;
    GpioJHead, GpioJ4;
    GpioJHead, GpioJ5;
    GpioJHead, GpioJ6;
    GpioJHead, GpioJ7;
    GpioJHead, GpioJ8;
    GpioJHead, GpioJ9;
    GpioJHead, GpioJ10;
    GpioJHead, GpioJ11;
    GpioJHead, GpioJ12;
    GpioJHead, GpioJ13;
    GpioJHead, GpioJ14;
    GpioJHead, GpioJ15;
);

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
pin_init!(
    GpioKHead, GpioK0;
    GpioKHead, GpioK1;
    GpioKHead, GpioK2;
    GpioKHead, GpioK3;
    GpioKHead, GpioK4;
    GpioKHead, GpioK5;
    GpioKHead, GpioK6;
    GpioKHead, GpioK7;
    GpioKHead, GpioK8;
    GpioKHead, GpioK9;
    GpioKHead, GpioK10;
    GpioKHead, GpioK11;
    GpioKHead, GpioK12;
    GpioKHead, GpioK13;
    GpioKHead, GpioK14;
    GpioKHead, GpioK15;
);
