use drone_cortexm::reg::prelude::*;
use drone_stm32_map::periph::uart::{UartMap, UartPeriph};

#[allow(dead_code)]
pub(crate) struct UartDiverged<Uart: UartMap> {
    pub(crate) rcc_busenr_uarten: Uart::SRccBusenrUarten,
    pub(crate) rcc_busrstr_uartrst: Uart::SRccBusrstrUartrst,
    pub(crate) rcc_bussmenr_uartsmen: Uart::SRccBussmenrUartsmen,
    pub(crate) uart_sr: Uart::CUartSr,
    pub(crate) uart_dr: Uart::SUartDr,
    pub(crate) uart_brr: Uart::SUartBrr,
    pub(crate) uart_cr1: Uart::SUartCr1,
    pub(crate) uart_cr2: Uart::SUartCr2,
    pub(crate) uart_cr3: Uart::SUartCr3,
    pub(crate) uart_gtpr: Uart::SUartGtprOpt,
}

impl<Uart: UartMap> From<UartPeriph<Uart>> for UartDiverged<Uart> {
    fn from(periph: UartPeriph<Uart>) -> Self {
        let UartPeriph {
            rcc_busenr_uarten,
            rcc_busrstr_uartrst,
            rcc_bussmenr_uartsmen,
            uart_sr,
            uart_dr,
            uart_brr,
            uart_cr1,
            uart_cr2,
            uart_cr3,
            uart_gtpr,
        } = periph;
        Self {
            rcc_busenr_uarten,
            rcc_busrstr_uartrst,
            rcc_bussmenr_uartsmen,
            uart_sr: uart_sr.into_copy(),
            uart_dr,
            uart_brr,
            uart_cr1,
            uart_cr2,
            uart_cr3,
            uart_gtpr,
        }
    }
}
