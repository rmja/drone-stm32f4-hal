use core::marker::PhantomData;
use drone_stm32_map::periph::uart::UartMap;
use drone_stm32f4_gpio_drv::pin_ext;

pub struct Defined;
pub struct Undefined;

pub struct UartPins<Uart: UartMap, Rx, Tx> {
    uart: PhantomData<Uart>,
    rx: PhantomData<Rx>,
    tx: PhantomData<Tx>,
}

impl<Uart: UartMap, Rx, Tx> UartPins<Uart, Rx, Tx> {
    pub fn new() -> Self {
        Self {
            uart: PhantomData,
            rx: PhantomData,
            tx: PhantomData,
        }
    }
}

impl<Uart: UartMap> Default for UartPins<Uart, Undefined, Undefined> {
    fn default() -> Self {
        Self::new()
    }
}

pin_ext!(RxPinExt<Uart: UartMap, ..., Rx, Tx>.rx -> UartPins<Uart, Defined, Tx>);
pin_ext!(TxPinExt<Uart: UartMap, ..., Rx, Tx>.tx -> UartPins<Uart, Rx, Defined>);
