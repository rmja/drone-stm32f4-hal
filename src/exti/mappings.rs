use crate::drv::{EdgeToken, ExtiDrv, ExtiLine};
use drone_stm32_map::periph::{exti::*, gpio::pin::*};

exti_line!(Exti0, GpioA0);
exti_line!(Exti0, GpioB0);
exti_line!(Exti0, GpioC0);
exti_line!(Exti0, GpioD0);
exti_line!(Exti0, GpioE0);
exti_line!(Exti0, GpioF0);
exti_line!(Exti0, GpioG0);
exti_line!(Exti0, GpioH0);
exti_line!(Exti0, GpioI0);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti0, GpioJ0);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti0, GpioK0);

exti_line!(Exti1, GpioA1);
exti_line!(Exti1, GpioB1);
exti_line!(Exti1, GpioC1);
exti_line!(Exti1, GpioD1);
exti_line!(Exti1, GpioE1);
exti_line!(Exti1, GpioF1);
exti_line!(Exti1, GpioG1);
exti_line!(Exti1, GpioH1);
exti_line!(Exti1, GpioI1);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti1, GpioJ1);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti1, GpioK1);

exti_line!(Exti2, GpioA2);
exti_line!(Exti2, GpioB2);
exti_line!(Exti2, GpioC2);
exti_line!(Exti2, GpioD2);
exti_line!(Exti2, GpioE2);
exti_line!(Exti2, GpioF2);
exti_line!(Exti2, GpioG2);
exti_line!(Exti2, GpioH2);
exti_line!(Exti2, GpioI2);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti2, GpioJ2);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti2, GpioK2);

exti_line!(Exti3, GpioA3);
exti_line!(Exti3, GpioB3);
exti_line!(Exti3, GpioC3);
exti_line!(Exti3, GpioD3);
exti_line!(Exti3, GpioE3);
exti_line!(Exti3, GpioF3);
exti_line!(Exti3, GpioG3);
exti_line!(Exti3, GpioH3);
exti_line!(Exti3, GpioI3);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti3, GpioJ3);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti3, GpioK3);

exti_line!(Exti4, GpioA4);
exti_line!(Exti4, GpioB4);
exti_line!(Exti4, GpioC4);
exti_line!(Exti4, GpioD4);
exti_line!(Exti4, GpioE4);
exti_line!(Exti4, GpioF4);
exti_line!(Exti4, GpioG4);
exti_line!(Exti4, GpioH4);
exti_line!(Exti4, GpioI4);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti4, GpioJ4);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti4, GpioK4);

exti_line!(Exti5, GpioA5);
exti_line!(Exti5, GpioB5);
exti_line!(Exti5, GpioC5);
exti_line!(Exti5, GpioD5);
exti_line!(Exti5, GpioE5);
exti_line!(Exti5, GpioF5);
exti_line!(Exti5, GpioG5);
exti_line!(Exti5, GpioH5);
exti_line!(Exti5, GpioI5);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti5, GpioJ5);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti5, GpioK5);

exti_line!(Exti6, GpioA6);
exti_line!(Exti6, GpioB6);
exti_line!(Exti6, GpioC6);
exti_line!(Exti6, GpioD6);
exti_line!(Exti6, GpioE6);
exti_line!(Exti6, GpioF6);
exti_line!(Exti6, GpioG6);
exti_line!(Exti6, GpioH6);
exti_line!(Exti6, GpioI6);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti6, GpioJ6);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti6, GpioK6);

exti_line!(Exti7, GpioA7);
exti_line!(Exti7, GpioB7);
exti_line!(Exti7, GpioC7);
exti_line!(Exti7, GpioD7);
exti_line!(Exti7, GpioE7);
exti_line!(Exti7, GpioF7);
exti_line!(Exti7, GpioG7);
exti_line!(Exti7, GpioH7);
exti_line!(Exti7, GpioI7);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti7, GpioJ7);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti7, GpioK7);

exti_line!(Exti8, GpioA8);
exti_line!(Exti8, GpioB8);
exti_line!(Exti8, GpioC8);
exti_line!(Exti8, GpioD8);
exti_line!(Exti8, GpioE8);
exti_line!(Exti8, GpioF8);
exti_line!(Exti8, GpioG8);
exti_line!(Exti8, GpioH8);
exti_line!(Exti8, GpioI8);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti8, GpioJ8);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti8, GpioK8);

exti_line!(Exti9, GpioA9);
exti_line!(Exti9, GpioB9);
exti_line!(Exti9, GpioC9);
exti_line!(Exti9, GpioD9);
exti_line!(Exti9, GpioE9);
exti_line!(Exti9, GpioF9);
exti_line!(Exti9, GpioG9);
exti_line!(Exti9, GpioH9);
exti_line!(Exti9, GpioI9);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti9, GpioJ9);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti9, GpioK9);

exti_line!(Exti10, GpioA10);
exti_line!(Exti10, GpioB10);
exti_line!(Exti10, GpioC10);
exti_line!(Exti10, GpioD10);
exti_line!(Exti10, GpioE10);
exti_line!(Exti10, GpioF10);
exti_line!(Exti10, GpioG10);
exti_line!(Exti10, GpioH10);
exti_line!(Exti10, GpioI10);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti10, GpioJ10);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti10, GpioK10);

exti_line!(Exti11, GpioA11);
exti_line!(Exti11, GpioB11);
exti_line!(Exti11, GpioC11);
exti_line!(Exti11, GpioD11);
exti_line!(Exti11, GpioE11);
exti_line!(Exti11, GpioF11);
exti_line!(Exti11, GpioG11);
exti_line!(Exti11, GpioH11);
exti_line!(Exti11, GpioI11);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti11, GpioJ11);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti11, GpioK11);

exti_line!(Exti12, GpioA12);
exti_line!(Exti12, GpioB12);
exti_line!(Exti12, GpioC12);
exti_line!(Exti12, GpioD12);
exti_line!(Exti12, GpioE12);
exti_line!(Exti12, GpioF12);
exti_line!(Exti12, GpioG12);
exti_line!(Exti12, GpioH12);
exti_line!(Exti12, GpioI12);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti12, GpioJ12);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti12, GpioK12);

exti_line!(Exti13, GpioA13);
exti_line!(Exti13, GpioB13);
exti_line!(Exti13, GpioC13);
exti_line!(Exti13, GpioD13);
exti_line!(Exti13, GpioE13);
exti_line!(Exti13, GpioF13);
exti_line!(Exti13, GpioG13);
exti_line!(Exti13, GpioH13);
exti_line!(Exti13, GpioI13);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti13, GpioJ13);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti13, GpioK13);

exti_line!(Exti14, GpioA14);
exti_line!(Exti14, GpioB14);
exti_line!(Exti14, GpioC14);
exti_line!(Exti14, GpioD14);
exti_line!(Exti14, GpioE14);
exti_line!(Exti14, GpioF14);
exti_line!(Exti14, GpioG14);
exti_line!(Exti14, GpioH14);
exti_line!(Exti14, GpioI14);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti14, GpioJ14);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti14, GpioK14);

exti_line!(Exti15, GpioA15);
exti_line!(Exti15, GpioB15);
exti_line!(Exti15, GpioC15);
exti_line!(Exti15, GpioD15);
exti_line!(Exti15, GpioE15);
exti_line!(Exti15, GpioF15);
exti_line!(Exti15, GpioG15);
exti_line!(Exti15, GpioH15);
exti_line!(Exti15, GpioI15);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti15, GpioJ15);
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
exti_line!(Exti15, GpioK15);