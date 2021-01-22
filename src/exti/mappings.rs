use crate::{line::HeadNum, exti_line};
use drone_stm32_map::periph::{exti::*, gpio::head::*, gpio::pin::*};

macro_rules! head_num {
    ($($head:ty, $num:expr;)+) => {
        $(
            impl HeadNum for $head {
                const NUM: u32 = $num;
            }
        )+
    }
}

head_num!(
    GpioAHead, 0;
    GpioBHead, 1;
    GpioCHead, 2;
    GpioDHead, 3;
    GpioEHead, 4;
    GpioFHead, 5;
    GpioGHead, 6;
    GpioHHead, 7;
    GpioIHead, 8;
);

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
head_num!(
    GpioJHead, 9;
    GpioKHead, 10;
);

exti_line!(
    Exti0, GpioAHead, GpioA0;
    Exti0, GpioBHead, GpioB0;
    Exti0, GpioCHead, GpioC0;
    Exti0, GpioDHead, GpioD0;
    Exti0, GpioEHead, GpioE0;
    Exti0, GpioFHead, GpioF0;
    Exti0, GpioGHead, GpioG0;
    Exti0, GpioHHead, GpioH0;
    Exti0, GpioIHead, GpioI0;
);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(
    Exti0, GpioJHead, GpioJ0;
    Exti0, GpioKHead, GpioK0;
);
exti_line!(
    Exti1, GpioAHead, GpioA1;
    Exti1, GpioBHead, GpioB1;
    Exti1, GpioCHead, GpioC1;
    Exti1, GpioDHead, GpioD1;
    Exti1, GpioEHead, GpioE1;
    Exti1, GpioFHead, GpioF1;
    Exti1, GpioGHead, GpioG1;
    Exti1, GpioHHead, GpioH1;
    Exti1, GpioIHead, GpioI1;
);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(
    Exti1, GpioJHead, GpioJ1;
    Exti1, GpioKHead, GpioK1;
);

exti_line!(
    Exti2, GpioAHead, GpioA2;
    Exti2, GpioBHead, GpioB2;
    Exti2, GpioCHead, GpioC2;
    Exti2, GpioDHead, GpioD2;
    Exti2, GpioEHead, GpioE2;
    Exti2, GpioFHead, GpioF2;
    Exti2, GpioGHead, GpioG2;
    Exti2, GpioHHead, GpioH2;
    Exti2, GpioIHead, GpioI2;
);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(
    Exti2, GpioJHead, GpioJ2;
    Exti2, GpioKHead, GpioK2;
);

exti_line!(
    Exti3, GpioAHead, GpioA3;
    Exti3, GpioBHead, GpioB3;
    Exti3, GpioCHead, GpioC3;
    Exti3, GpioDHead, GpioD3;
    Exti3, GpioEHead, GpioE3;
    Exti3, GpioFHead, GpioF3;
    Exti3, GpioGHead, GpioG3;
    Exti3, GpioHHead, GpioH3;
    Exti3, GpioIHead, GpioI3;
);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(
    Exti3, GpioJHead, GpioJ3;
    Exti3, GpioKHead, GpioK3;
);

exti_line!(
    Exti4, GpioAHead, GpioA4;
    Exti4, GpioBHead, GpioB4;
    Exti4, GpioCHead, GpioC4;
    Exti4, GpioDHead, GpioD4;
    Exti4, GpioEHead, GpioE4;
    Exti4, GpioFHead, GpioF4;
    Exti4, GpioGHead, GpioG4;
    Exti4, GpioHHead, GpioH4;
    Exti4, GpioIHead, GpioI4;
);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(
    Exti4, GpioJHead, GpioJ4;
    Exti4, GpioKHead, GpioK4;
);

exti_line!(
    Exti5, GpioAHead, GpioA5;
    Exti5, GpioBHead, GpioB5;
    Exti5, GpioCHead, GpioC5;
    Exti5, GpioDHead, GpioD5;
    Exti5, GpioEHead, GpioE5;
    Exti5, GpioFHead, GpioF5;
    Exti5, GpioGHead, GpioG5;
    Exti5, GpioHHead, GpioH5;
    Exti5, GpioIHead, GpioI5;
);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(
    Exti5, GpioJHead, GpioJ5;
    Exti5, GpioKHead, GpioK5;
);

exti_line!(
    Exti6, GpioAHead, GpioA6;
    Exti6, GpioBHead, GpioB6;
    Exti6, GpioCHead, GpioC6;
    Exti6, GpioDHead, GpioD6;
    Exti6, GpioEHead, GpioE6;
    Exti6, GpioFHead, GpioF6;
    Exti6, GpioGHead, GpioG6;
    Exti6, GpioHHead, GpioH6;
    Exti6, GpioIHead, GpioI6;
);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(
    Exti6, GpioJHead, GpioJ6;
    Exti6, GpioKHead, GpioK6;
);

exti_line!(
    Exti7, GpioAHead, GpioA7;
    Exti7, GpioBHead, GpioB7;
    Exti7, GpioCHead, GpioC7;
    Exti7, GpioDHead, GpioD7;
    Exti7, GpioEHead, GpioE7;
    Exti7, GpioFHead, GpioF7;
    Exti7, GpioGHead, GpioG7;
    Exti7, GpioHHead, GpioH7;
    Exti7, GpioIHead, GpioI7;
);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(
    Exti7, GpioJHead, GpioJ7;
    Exti7, GpioKHead, GpioK7;
);

exti_line!(
    Exti8, GpioAHead, GpioA8;
    Exti8, GpioBHead, GpioB8;
    Exti8, GpioCHead, GpioC8;
    Exti8, GpioDHead, GpioD8;
    Exti8, GpioEHead, GpioE8;
    Exti8, GpioFHead, GpioF8;
    Exti8, GpioGHead, GpioG8;
    Exti8, GpioHHead, GpioH8;
    Exti8, GpioIHead, GpioI8;
);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(
    Exti8, GpioJHead, GpioJ8;
    Exti8, GpioKHead, GpioK8;
);

exti_line!(
    Exti9, GpioAHead, GpioA9;
    Exti9, GpioBHead, GpioB9;
    Exti9, GpioCHead, GpioC9;
    Exti9, GpioDHead, GpioD9;
    Exti9, GpioEHead, GpioE9;
    Exti9, GpioFHead, GpioF9;
    Exti9, GpioGHead, GpioG9;
    Exti9, GpioHHead, GpioH9;
    Exti9, GpioIHead, GpioI9;
);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(
    Exti9, GpioJHead, GpioJ9;
    Exti9, GpioKHead, GpioK9;
);

exti_line!(
    Exti10, GpioAHead, GpioA10;
    Exti10, GpioBHead, GpioB10;
    Exti10, GpioCHead, GpioC10;
    Exti10, GpioDHead, GpioD10;
    Exti10, GpioEHead, GpioE10;
    Exti10, GpioFHead, GpioF10;
    Exti10, GpioGHead, GpioG10;
    Exti10, GpioHHead, GpioH10;
    Exti10, GpioIHead, GpioI10;
);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(
    Exti10, GpioJHead, GpioJ10;
    Exti10, GpioKHead, GpioK10;
);

exti_line!(
    Exti11, GpioAHead, GpioA11;
    Exti11, GpioBHead, GpioB11;
    Exti11, GpioCHead, GpioC11;
    Exti11, GpioDHead, GpioD11;
    Exti11, GpioEHead, GpioE11;
    Exti11, GpioFHead, GpioF11;
    Exti11, GpioGHead, GpioG11;
    Exti11, GpioHHead, GpioH11;
    Exti11, GpioIHead, GpioI11;
);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(
    Exti11, GpioJHead, GpioJ11;
    Exti11, GpioKHead, GpioK11;
);

exti_line!(
    Exti12, GpioAHead, GpioA12;
    Exti12, GpioBHead, GpioB12;
    Exti12, GpioCHead, GpioC12;
    Exti12, GpioDHead, GpioD12;
    Exti12, GpioEHead, GpioE12;
    Exti12, GpioFHead, GpioF12;
    Exti12, GpioGHead, GpioG12;
    Exti12, GpioHHead, GpioH12;
    Exti12, GpioIHead, GpioI12;
);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(
    Exti12, GpioJHead, GpioJ12;
    Exti12, GpioKHead, GpioK12;
);

exti_line!(
    Exti13, GpioAHead, GpioA13;
    Exti13, GpioBHead, GpioB13;
    Exti13, GpioCHead, GpioC13;
    Exti13, GpioDHead, GpioD13;
    Exti13, GpioEHead, GpioE13;
    Exti13, GpioFHead, GpioF13;
    Exti13, GpioGHead, GpioG13;
    Exti13, GpioHHead, GpioH13;
    Exti13, GpioIHead, GpioI13;
);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(
    Exti13, GpioJHead, GpioJ13;
    Exti13, GpioKHead, GpioK13;
);

exti_line!(
    Exti14, GpioAHead, GpioA14;
    Exti14, GpioBHead, GpioB14;
    Exti14, GpioCHead, GpioC14;
    Exti14, GpioDHead, GpioD14;
    Exti14, GpioEHead, GpioE14;
    Exti14, GpioFHead, GpioF14;
    Exti14, GpioGHead, GpioG14;
    Exti14, GpioHHead, GpioH14;
    Exti14, GpioIHead, GpioI14;
);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(
    Exti14, GpioJHead, GpioJ14;
    Exti14, GpioKHead, GpioK14;
);

exti_line!(
    Exti15, GpioAHead, GpioA15;
    Exti15, GpioBHead, GpioB15;
    Exti15, GpioCHead, GpioC15;
    Exti15, GpioDHead, GpioD15;
    Exti15, GpioEHead, GpioE15;
    Exti15, GpioFHead, GpioF15;
    Exti15, GpioGHead, GpioG15;
    Exti15, GpioHHead, GpioH15;
    Exti15, GpioIHead, GpioI15;
);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(
    Exti15, GpioJHead, GpioJ15;
    Exti15, GpioKHead, GpioK15;
);
