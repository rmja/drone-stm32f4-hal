//! The root task.

use crate::{thr, thr::ThrsInit, Regs};
use drone_cortexm::{reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::{
    dma::{
        periph_dma2,
        periph_dma2_ch2,
        periph_dma2_ch3,
    },
    gpio::{
        periph_gpio_a_head,
        periph_gpio_a5,
        periph_gpio_a6,
        periph_gpio_a7,
    },
    spi::{periph_spi1},
};
use drone_stm32f4_hal::spi::{SpiDrv, config::*};

/// The root task handler.
#[inline(never)]
pub fn handler(reg: Regs, thr_init: ThrsInit) {
    let thr = thr::init(thr_init);

    thr.hard_fault.add_once(|| panic!("Hard Fault"));

    println!("Hello, world!");


    let setup = SpiSetup::default(periph_spi1!(reg), thr.spi_1).at(Prsc::Prsc16);
    let mut spi_drv = SpiDrv::init(setup);
    let mut spi_master = spi_drv.master();

    let tx_buf = [1,2,3,4].as_ref();
    spi_master.send(tx_buf).root_wait();

    // Enter a sleep state on ISR exit.
    reg.scb_scr.sleeponexit.set_bit();
}
