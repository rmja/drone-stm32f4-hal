use crate::{drv::{EdgeToken, ExtiDrv}, line::HeadNum};
use drone_stm32_map::periph::{exti::*, gpio::head::*, gpio::pin::*};

impl HeadNum for GpioAHead {
    fn num() -> u32 { 0 }
}

impl HeadNum for GpioBHead {
    fn num() -> u32 { 1 }
}

impl HeadNum for GpioCHead {
    fn num() -> u32 { 2 }
}

impl HeadNum for GpioDHead {
    fn num() -> u32 { 3 }
}

impl HeadNum for GpioEHead {
    fn num() -> u32 { 4 }
}

impl HeadNum for GpioFHead {
    fn num() -> u32 { 5 }
}

impl HeadNum for GpioGHead {
    fn num() -> u32 { 6 }
}

impl HeadNum for GpioHHead {
    fn num() -> u32 { 7 }
}

impl HeadNum for GpioIHead {
    fn num() -> u32 { 8 }
}

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
impl HeadNum for GpioJHead {
    fn num() -> u32 { 9 }
}

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
impl HeadNum for GpioKHead {
    fn num() -> u32 { 10 }
}


exti_line!(Exti0, GpioAHead, GpioA0);
exti_line!(Exti0, GpioBHead, GpioB0);
exti_line!(Exti0, GpioCHead, GpioC0);
exti_line!(Exti0, GpioDHead, GpioD0);
exti_line!(Exti0, GpioEHead, GpioE0);
exti_line!(Exti0, GpioFHead, GpioF0);
exti_line!(Exti0, GpioGHead, GpioG0);
exti_line!(Exti0, GpioHHead, GpioH0);
exti_line!(Exti0, GpioIHead, GpioI0);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti0, GpioJHead, GpioJ0);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti0, GpioKHead, GpioK0);

exti_line!(Exti1, GpioAHead, GpioA1);
exti_line!(Exti1, GpioBHead, GpioB1);
exti_line!(Exti1, GpioCHead, GpioC1);
exti_line!(Exti1, GpioDHead, GpioD1);
exti_line!(Exti1, GpioEHead, GpioE1);
exti_line!(Exti1, GpioFHead, GpioF1);
exti_line!(Exti1, GpioGHead, GpioG1);
exti_line!(Exti1, GpioHHead, GpioH1);
exti_line!(Exti1, GpioIHead, GpioI1);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti1, GpioJHead, GpioJ1);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti1, GpioKHead, GpioK1);

exti_line!(Exti2, GpioAHead, GpioA2);
exti_line!(Exti2, GpioBHead, GpioB2);
exti_line!(Exti2, GpioCHead, GpioC2);
exti_line!(Exti2, GpioDHead, GpioD2);
exti_line!(Exti2, GpioEHead, GpioE2);
exti_line!(Exti2, GpioFHead, GpioF2);
exti_line!(Exti2, GpioGHead, GpioG2);
exti_line!(Exti2, GpioHHead, GpioH2);
exti_line!(Exti2, GpioIHead, GpioI2);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti2, GpioJHead, GpioJ2);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti2, GpioKHead, GpioK2);

exti_line!(Exti3, GpioAHead, GpioA3);
exti_line!(Exti3, GpioBHead, GpioB3);
exti_line!(Exti3, GpioCHead, GpioC3);
exti_line!(Exti3, GpioDHead, GpioD3);
exti_line!(Exti3, GpioEHead, GpioE3);
exti_line!(Exti3, GpioFHead, GpioF3);
exti_line!(Exti3, GpioGHead, GpioG3);
exti_line!(Exti3, GpioHHead, GpioH3);
exti_line!(Exti3, GpioIHead, GpioI3);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti3, GpioJHead, GpioJ3);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti3, GpioKHead, GpioK3);

exti_line!(Exti4, GpioAHead, GpioA4);
exti_line!(Exti4, GpioBHead, GpioB4);
exti_line!(Exti4, GpioCHead, GpioC4);
exti_line!(Exti4, GpioDHead, GpioD4);
exti_line!(Exti4, GpioEHead, GpioE4);
exti_line!(Exti4, GpioFHead, GpioF4);
exti_line!(Exti4, GpioGHead, GpioG4);
exti_line!(Exti4, GpioHHead, GpioH4);
exti_line!(Exti4, GpioIHead, GpioI4);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti4, GpioJHead, GpioJ4);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti4, GpioKHead, GpioK4);

exti_line!(Exti5, GpioAHead, GpioA5);
exti_line!(Exti5, GpioBHead, GpioB5);
exti_line!(Exti5, GpioCHead, GpioC5);
exti_line!(Exti5, GpioDHead, GpioD5);
exti_line!(Exti5, GpioEHead, GpioE5);
exti_line!(Exti5, GpioFHead, GpioF5);
exti_line!(Exti5, GpioGHead, GpioG5);
exti_line!(Exti5, GpioHHead, GpioH5);
exti_line!(Exti5, GpioIHead, GpioI5);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti5, GpioJHead, GpioJ5);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti5, GpioKHead, GpioK5);

exti_line!(Exti6, GpioAHead, GpioA6);
exti_line!(Exti6, GpioBHead, GpioB6);
exti_line!(Exti6, GpioCHead, GpioC6);
exti_line!(Exti6, GpioDHead, GpioD6);
exti_line!(Exti6, GpioEHead, GpioE6);
exti_line!(Exti6, GpioFHead, GpioF6);
exti_line!(Exti6, GpioGHead, GpioG6);
exti_line!(Exti6, GpioHHead, GpioH6);
exti_line!(Exti6, GpioIHead, GpioI6);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti6, GpioJHead, GpioJ6);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti6, GpioKHead, GpioK6);

exti_line!(Exti7, GpioAHead, GpioA7);
exti_line!(Exti7, GpioBHead, GpioB7);
exti_line!(Exti7, GpioCHead, GpioC7);
exti_line!(Exti7, GpioDHead, GpioD7);
exti_line!(Exti7, GpioEHead, GpioE7);
exti_line!(Exti7, GpioFHead, GpioF7);
exti_line!(Exti7, GpioGHead, GpioG7);
exti_line!(Exti7, GpioHHead, GpioH7);
exti_line!(Exti7, GpioIHead, GpioI7);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti7, GpioJHead, GpioJ7);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti7, GpioKHead, GpioK7);

exti_line!(Exti8, GpioAHead, GpioA8);
exti_line!(Exti8, GpioBHead, GpioB8);
exti_line!(Exti8, GpioCHead, GpioC8);
exti_line!(Exti8, GpioDHead, GpioD8);
exti_line!(Exti8, GpioEHead, GpioE8);
exti_line!(Exti8, GpioFHead, GpioF8);
exti_line!(Exti8, GpioGHead, GpioG8);
exti_line!(Exti8, GpioHHead, GpioH8);
exti_line!(Exti8, GpioIHead, GpioI8);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti8, GpioJHead, GpioJ8);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti8, GpioKHead, GpioK8);

exti_line!(Exti9, GpioAHead, GpioA9);
exti_line!(Exti9, GpioBHead, GpioB9);
exti_line!(Exti9, GpioCHead, GpioC9);
exti_line!(Exti9, GpioDHead, GpioD9);
exti_line!(Exti9, GpioEHead, GpioE9);
exti_line!(Exti9, GpioFHead, GpioF9);
exti_line!(Exti9, GpioGHead, GpioG9);
exti_line!(Exti9, GpioHHead, GpioH9);
exti_line!(Exti9, GpioIHead, GpioI9);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti9, GpioJHead, GpioJ9);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti9, GpioKHead, GpioK9);

exti_line!(Exti10, GpioAHead, GpioA10);
exti_line!(Exti10, GpioBHead, GpioB10);
exti_line!(Exti10, GpioCHead, GpioC10);
exti_line!(Exti10, GpioDHead, GpioD10);
exti_line!(Exti10, GpioEHead, GpioE10);
exti_line!(Exti10, GpioFHead, GpioF10);
exti_line!(Exti10, GpioGHead, GpioG10);
exti_line!(Exti10, GpioHHead, GpioH10);
exti_line!(Exti10, GpioIHead, GpioI10);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti10, GpioJHead, GpioJ10);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti10, GpioKHead, GpioK10);

exti_line!(Exti11, GpioAHead, GpioA11);
exti_line!(Exti11, GpioBHead, GpioB11);
exti_line!(Exti11, GpioCHead, GpioC11);
exti_line!(Exti11, GpioDHead, GpioD11);
exti_line!(Exti11, GpioEHead, GpioE11);
exti_line!(Exti11, GpioFHead, GpioF11);
exti_line!(Exti11, GpioGHead, GpioG11);
exti_line!(Exti11, GpioHHead, GpioH11);
exti_line!(Exti11, GpioIHead, GpioI11);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti11, GpioJHead, GpioJ11);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti11, GpioKHead, GpioK11);

exti_line!(Exti12, GpioAHead, GpioA12);
exti_line!(Exti12, GpioBHead, GpioB12);
exti_line!(Exti12, GpioCHead, GpioC12);
exti_line!(Exti12, GpioDHead, GpioD12);
exti_line!(Exti12, GpioEHead, GpioE12);
exti_line!(Exti12, GpioFHead, GpioF12);
exti_line!(Exti12, GpioGHead, GpioG12);
exti_line!(Exti12, GpioHHead, GpioH12);
exti_line!(Exti12, GpioIHead, GpioI12);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti12, GpioJHead, GpioJ12);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti12, GpioKHead, GpioK12);

exti_line!(Exti13, GpioAHead, GpioA13);
exti_line!(Exti13, GpioBHead, GpioB13);
exti_line!(Exti13, GpioCHead, GpioC13);
exti_line!(Exti13, GpioDHead, GpioD13);
exti_line!(Exti13, GpioEHead, GpioE13);
exti_line!(Exti13, GpioFHead, GpioF13);
exti_line!(Exti13, GpioGHead, GpioG13);
exti_line!(Exti13, GpioHHead, GpioH13);
exti_line!(Exti13, GpioIHead, GpioI13);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti13, GpioJHead, GpioJ13);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti13, GpioKHead, GpioK13);

exti_line!(Exti14, GpioAHead, GpioA14);
exti_line!(Exti14, GpioBHead, GpioB14);
exti_line!(Exti14, GpioCHead, GpioC14);
exti_line!(Exti14, GpioDHead, GpioD14);
exti_line!(Exti14, GpioEHead, GpioE14);
exti_line!(Exti14, GpioFHead, GpioF14);
exti_line!(Exti14, GpioGHead, GpioG14);
exti_line!(Exti14, GpioHHead, GpioH14);
exti_line!(Exti14, GpioIHead, GpioI14);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti14, GpioJHead, GpioJ14);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti14, GpioKHead, GpioK14);

exti_line!(Exti15, GpioAHead, GpioA15);
exti_line!(Exti15, GpioBHead, GpioB15);
exti_line!(Exti15, GpioCHead, GpioC15);
exti_line!(Exti15, GpioDHead, GpioD15);
exti_line!(Exti15, GpioEHead, GpioE15);
exti_line!(Exti15, GpioFHead, GpioF15);
exti_line!(Exti15, GpioGHead, GpioG15);
exti_line!(Exti15, GpioHHead, GpioH15);
exti_line!(Exti15, GpioIHead, GpioI15);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti15, GpioJHead, GpioJ15);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti15, GpioKHead, GpioK15);