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
        periph_gpio_b_head,
        periph_gpio_b0,
    },
    spi::{periph_spi1},
};
use drone_stm32f4_hal::{gpio::{GpioPinCfg, GpioPinSpeed}, spi::{IfaceRoot, SpiDrv, SpiIface, config::*}};

/// The root task handler.
#[inline(never)]
pub fn handler(reg: Regs, thr_init: ThrsInit) {
    let thr = thr::init(thr_init);

    thr.hard_fault.add_once(|| panic!("Hard Fault"));

    println!("Hello, world!");


    let setup = SpiSetup::new(
        periph_spi1!(reg), 
        thr.spi_1,
        BaudRate::max(10_000_000, 90_000_000)
    );
    let mut spi_drv = SpiDrv::init(setup);
    let mut spi_master = spi_drv.master();

    let cs = GpioPinCfg::from(periph_gpio_b0!(reg)).into_output().with_speed(GpioPinSpeed::HighSpeed).pin();
    let iface = SpiIface::new(cs);

    spi_master.select(&iface);
    let tx_buf = [1,2,3,4].as_ref();
    spi_master.send(tx_buf).root_wait();
    spi_master.deselect(&iface);

    // Enter a sleep state on ISR exit.
    reg.scb_scr.sleeponexit.set_bit();
}
