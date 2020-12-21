//! The root task.

use crate::{thr, thr::ThrsInit, Regs};
use drone_cortexm::{reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::{
    exti::periph_exti4,
    gpio::{
        periph_gpio_b_head,
        periph_gpio_b4,
    },
};
use drone_stm32f4_hal::{exti::{ExtiDrv, ExtiSetup, prelude::*}, gpio::{prelude::*, GpioHead}};

/// The root task handler.
#[inline(never)]
pub fn handler(reg: Regs, thr_init: ThrsInit) {
    let thr = thr::init(thr_init);

    thr.hard_fault.add_once(|| panic!("Hard Fault"));

    println!("Hello, world!");

    // Enable interrupts.
    thr.exti_4.enable_int();

    let gpio = GpioHead::with_enabled_clock(periph_gpio_b_head!(reg));
    let pin = gpio.pin(periph_gpio_b4!(reg)).into_input();

    let setup = ExtiSetup::new(periph_exti4!(reg), thr.exti_4);
    let exti = ExtiDrv::init(setup).into_rising_edge();

    let line = exti.line(pin);


    // Enter a sleep state on ISR exit.
    reg.scb_scr.sleeponexit.set_bit();
}