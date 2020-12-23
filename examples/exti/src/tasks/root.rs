//! The root task.

use crate::{thr, thr::ThrsInit, Regs};
use drone_cortexm::{reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::{
    exti::periph_exti2,
    gpio::{
        periph_gpio_i_head,
        periph_gpio_i2,
    },
};
use drone_stm32f4_hal::{exti::{ExtiDrv, ExtiSetup, Syscfg, prelude::*,periph_syscfg}, gpio::{prelude::*, GpioHead}};
use futures::prelude::*;

/// The root task handler.
#[inline(never)]
pub fn handler(reg: Regs, thr_init: ThrsInit) {
    let thr = thr::init(thr_init);

    thr.hard_fault.add_once(|| panic!("Hard Fault"));

    println!("Hello, world!");

    // Enable interrupts.
    thr.exti_2.enable_int();

    let gpio = GpioHead::with_enabled_clock(periph_gpio_i_head!(reg));
    let pin = gpio.pin(periph_gpio_i2!(reg)).into_input().into_pp().into_pulldown();

    // unsafe {
    //     gpio.disable_clock();
    // }

    let setup = ExtiSetup::new(periph_exti2!(reg), thr.exti_2);
    let syscfg = Syscfg::with_enabled_clock(periph_syscfg!(reg));
    let exti = ExtiDrv::init(setup, &syscfg).into_rising_edge();

    pin.get();

    let line = exti.line(&pin);
    while let Some(tick) = line.create_saturating_stream().next().root_wait() {
        let _ = pin.get();
    }

    // Enter a sleep state on ISR exit.
    reg.scb_scr.sleeponexit.set_bit();
}