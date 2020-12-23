use crate::drv::{EdgeToken, ExtiDrv};
use drone_stm32_map::periph::{exti::*, gpio::*, gpio::pin::*};

trait PortNum {
    fn num() -> u32;
}

impl PortNum for GpioA {
    fn num() -> u32 { 0 }
}

impl PortNum for GpioB {
    fn num() -> u32 { 1 }
}

impl PortNum for GpioC {
    fn num() -> u32 { 2 }
}

impl PortNum for GpioD {
    fn num() -> u32 { 3 }
}

impl PortNum for GpioE {
    fn num() -> u32 { 4 }
}

impl PortNum for GpioF {
    fn num() -> u32 { 5 }
}

impl PortNum for GpioG {
    fn num() -> u32 { 6 }
}

impl PortNum for GpioH {
    fn num() -> u32 { 7 }
}

impl PortNum for GpioI {
    fn num() -> u32 { 8 }
}

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
impl PortNum for GpioJ {
    fn num() -> u32 { 9 }
}

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
impl PortNum for GpioK {
    fn num() -> u32 { 10 }
}


exti_line!(Exti0, GpioA, GpioA0);
exti_line!(Exti0, GpioB, GpioB0);
exti_line!(Exti0, GpioC, GpioC0);
exti_line!(Exti0, GpioD, GpioD0);
exti_line!(Exti0, GpioE, GpioE0);
exti_line!(Exti0, GpioF, GpioF0);
exti_line!(Exti0, GpioG, GpioG0);
exti_line!(Exti0, GpioH, GpioH0);
exti_line!(Exti0, GpioI, GpioI0);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti0, GpioJ, GpioJ0);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti0, GpioK, GpioK0);

exti_line!(Exti1, GpioA, GpioA1);
exti_line!(Exti1, GpioB, GpioB1);
exti_line!(Exti1, GpioC, GpioC1);
exti_line!(Exti1, GpioD, GpioD1);
exti_line!(Exti1, GpioE, GpioE1);
exti_line!(Exti1, GpioF, GpioF1);
exti_line!(Exti1, GpioG, GpioG1);
exti_line!(Exti1, GpioH, GpioH1);
exti_line!(Exti1, GpioI, GpioI1);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti1, GpioJ, GpioJ1);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti1, GpioK, GpioK1);

exti_line!(Exti2, GpioA, GpioA2);
exti_line!(Exti2, GpioB, GpioB2);
exti_line!(Exti2, GpioC, GpioC2);
exti_line!(Exti2, GpioD, GpioD2);
exti_line!(Exti2, GpioE, GpioE2);
exti_line!(Exti2, GpioF, GpioF2);
exti_line!(Exti2, GpioG, GpioG2);
exti_line!(Exti2, GpioH, GpioH2);
exti_line!(Exti2, GpioI, GpioI2);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti2, GpioJ, GpioJ2);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti2, GpioK, GpioK2);

exti_line!(Exti3, GpioA, GpioA3);
exti_line!(Exti3, GpioB, GpioB3);
exti_line!(Exti3, GpioC, GpioC3);
exti_line!(Exti3, GpioD, GpioD3);
exti_line!(Exti3, GpioE, GpioE3);
exti_line!(Exti3, GpioF, GpioF3);
exti_line!(Exti3, GpioG, GpioG3);
exti_line!(Exti3, GpioH, GpioH3);
exti_line!(Exti3, GpioI, GpioI3);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti3, GpioJ, GpioJ3);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti3, GpioK, GpioK3);

exti_line!(Exti4, GpioA, GpioA4);
exti_line!(Exti4, GpioB, GpioB4);
exti_line!(Exti4, GpioC, GpioC4);
exti_line!(Exti4, GpioD, GpioD4);
exti_line!(Exti4, GpioE, GpioE4);
exti_line!(Exti4, GpioF, GpioF4);
exti_line!(Exti4, GpioG, GpioG4);
exti_line!(Exti4, GpioH, GpioH4);
exti_line!(Exti4, GpioI, GpioI4);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti4, GpioJ, GpioJ4);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti4, GpioK, GpioK4);

exti_line!(Exti5, GpioA, GpioA5);
exti_line!(Exti5, GpioB, GpioB5);
exti_line!(Exti5, GpioC, GpioC5);
exti_line!(Exti5, GpioD, GpioD5);
exti_line!(Exti5, GpioE, GpioE5);
exti_line!(Exti5, GpioF, GpioF5);
exti_line!(Exti5, GpioG, GpioG5);
exti_line!(Exti5, GpioH, GpioH5);
exti_line!(Exti5, GpioI, GpioI5);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti5, GpioJ, GpioJ5);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti5, GpioK, GpioK5);

exti_line!(Exti6, GpioA, GpioA6);
exti_line!(Exti6, GpioB, GpioB6);
exti_line!(Exti6, GpioC, GpioC6);
exti_line!(Exti6, GpioD, GpioD6);
exti_line!(Exti6, GpioE, GpioE6);
exti_line!(Exti6, GpioF, GpioF6);
exti_line!(Exti6, GpioG, GpioG6);
exti_line!(Exti6, GpioH, GpioH6);
exti_line!(Exti6, GpioI, GpioI6);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti6, GpioJ, GpioJ6);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti6, GpioK, GpioK6);

exti_line!(Exti7, GpioA, GpioA7);
exti_line!(Exti7, GpioB, GpioB7);
exti_line!(Exti7, GpioC, GpioC7);
exti_line!(Exti7, GpioD, GpioD7);
exti_line!(Exti7, GpioE, GpioE7);
exti_line!(Exti7, GpioF, GpioF7);
exti_line!(Exti7, GpioG, GpioG7);
exti_line!(Exti7, GpioH, GpioH7);
exti_line!(Exti7, GpioI, GpioI7);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti7, GpioJ, GpioJ7);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti7, GpioK, GpioK7);

exti_line!(Exti8, GpioA, GpioA8);
exti_line!(Exti8, GpioB, GpioB8);
exti_line!(Exti8, GpioC, GpioC8);
exti_line!(Exti8, GpioD, GpioD8);
exti_line!(Exti8, GpioE, GpioE8);
exti_line!(Exti8, GpioF, GpioF8);
exti_line!(Exti8, GpioG, GpioG8);
exti_line!(Exti8, GpioH, GpioH8);
exti_line!(Exti8, GpioI, GpioI8);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti8, GpioJ, GpioJ8);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti8, GpioK, GpioK8);

exti_line!(Exti9, GpioA, GpioA9);
exti_line!(Exti9, GpioB, GpioB9);
exti_line!(Exti9, GpioC, GpioC9);
exti_line!(Exti9, GpioD, GpioD9);
exti_line!(Exti9, GpioE, GpioE9);
exti_line!(Exti9, GpioF, GpioF9);
exti_line!(Exti9, GpioG, GpioG9);
exti_line!(Exti9, GpioH, GpioH9);
exti_line!(Exti9, GpioI, GpioI9);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti9, GpioJ, GpioJ9);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti9, GpioK, GpioK9);

exti_line!(Exti10, GpioA, GpioA10);
exti_line!(Exti10, GpioB, GpioB10);
exti_line!(Exti10, GpioC, GpioC10);
exti_line!(Exti10, GpioD, GpioD10);
exti_line!(Exti10, GpioE, GpioE10);
exti_line!(Exti10, GpioF, GpioF10);
exti_line!(Exti10, GpioG, GpioG10);
exti_line!(Exti10, GpioH, GpioH10);
exti_line!(Exti10, GpioI, GpioI10);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti10, GpioJ, GpioJ10);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti10, GpioK, GpioK10);

exti_line!(Exti11, GpioA, GpioA11);
exti_line!(Exti11, GpioB, GpioB11);
exti_line!(Exti11, GpioC, GpioC11);
exti_line!(Exti11, GpioD, GpioD11);
exti_line!(Exti11, GpioE, GpioE11);
exti_line!(Exti11, GpioF, GpioF11);
exti_line!(Exti11, GpioG, GpioG11);
exti_line!(Exti11, GpioH, GpioH11);
exti_line!(Exti11, GpioI, GpioI11);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti11, GpioJ, GpioJ11);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti11, GpioK, GpioK11);

exti_line!(Exti12, GpioA, GpioA12);
exti_line!(Exti12, GpioB, GpioB12);
exti_line!(Exti12, GpioC, GpioC12);
exti_line!(Exti12, GpioD, GpioD12);
exti_line!(Exti12, GpioE, GpioE12);
exti_line!(Exti12, GpioF, GpioF12);
exti_line!(Exti12, GpioG, GpioG12);
exti_line!(Exti12, GpioH, GpioH12);
exti_line!(Exti12, GpioI, GpioI12);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti12, GpioJ, GpioJ12);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti12, GpioK, GpioK12);

exti_line!(Exti13, GpioA, GpioA13);
exti_line!(Exti13, GpioB, GpioB13);
exti_line!(Exti13, GpioC, GpioC13);
exti_line!(Exti13, GpioD, GpioD13);
exti_line!(Exti13, GpioE, GpioE13);
exti_line!(Exti13, GpioF, GpioF13);
exti_line!(Exti13, GpioG, GpioG13);
exti_line!(Exti13, GpioH, GpioH13);
exti_line!(Exti13, GpioI, GpioI13);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti13, GpioJ, GpioJ13);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti13, GpioK, GpioK13);

exti_line!(Exti14, GpioA, GpioA14);
exti_line!(Exti14, GpioB, GpioB14);
exti_line!(Exti14, GpioC, GpioC14);
exti_line!(Exti14, GpioD, GpioD14);
exti_line!(Exti14, GpioE, GpioE14);
exti_line!(Exti14, GpioF, GpioF14);
exti_line!(Exti14, GpioG, GpioG14);
exti_line!(Exti14, GpioH, GpioH14);
exti_line!(Exti14, GpioI, GpioI14);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti14, GpioJ, GpioJ14);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti14, GpioK, GpioK14);

exti_line!(Exti15, GpioA, GpioA15);
exti_line!(Exti15, GpioB, GpioB15);
exti_line!(Exti15, GpioC, GpioC15);
exti_line!(Exti15, GpioD, GpioD15);
exti_line!(Exti15, GpioE, GpioE15);
exti_line!(Exti15, GpioF, GpioF15);
exti_line!(Exti15, GpioG, GpioG15);
exti_line!(Exti15, GpioH, GpioH15);
exti_line!(Exti15, GpioI, GpioI15);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti15, GpioJ, GpioJ15);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti15, GpioK, GpioK15);