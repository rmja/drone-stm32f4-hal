use core::marker::PhantomData;
use drone_stm32_map::periph::{gpio::pin::GpioPinMap, uart::UartMap};
use drone_stm32f4_gpio_drv::{pin_ext, prelude::*, GpioPin};

pub struct Defined;
pub struct Undefined;

pub struct UartPins<Uart: UartMap, Rx, Tx> {
    uart: PhantomData<Uart>,
    rx: PhantomData<Rx>,
    tx: PhantomData<Tx>,
}

impl<Uart: UartMap> UartPins<Uart, Undefined, Undefined> {
    pub fn new() -> UartPins<Uart, Undefined, Undefined> {
        UartPins::default()
    }
}

impl<Uart: UartMap, Rx, Tx> Default for UartPins<Uart, Rx, Tx> {
    fn default() -> Self {
        Self {
            uart: PhantomData,
            rx: PhantomData,
            tx: PhantomData,
        }
    }
}

pin_ext!(RxPinExt<Uart: UartMap, ..., Rx, Tx>.rx -> UartPins<Uart, Defined, Tx>);
pin_ext!(TxPinExt<Uart: UartMap, ..., Rx, Tx>.tx -> UartPins<Uart, Rx, Defined>);
