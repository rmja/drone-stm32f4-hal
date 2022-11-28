//! The root task.

use crate::{thr, CoreRegs, Regs};
use drone_cortexm::periph_thr;
use drone_cortexm::reg::prelude::*;
use drone_cortexm::thr::prelude::*;
use drone_stm32_map::periph::{
    exti::periph_exti2,
    gpio::{periph_gpio_i2, periph_gpio_i_head},
};
use drone_stm32f4_hal::{
    exti::{periph_syscfg, prelude::*, ExtiDrv, Syscfg},
    gpio::{prelude::*, GpioHead},
};

/// The root task handler.
#[inline(never)]
#[export_name = "root"]
pub fn handler(reg: Regs, core_reg: CoreRegs, thr: thr::Init) {
    let thr = thr.init(periph_thr!(core_reg));

    thr.hard_fault.add_once(|| panic!("Hard Fault"));

    println!("Hello, world!");

    // Enable interrupts.
    thr.exti2.enable_int();

    let gpio = GpioHead::with_enabled_clock(periph_gpio_i_head!(reg));
    let pin = gpio
        .pin(periph_gpio_i2!(reg))
        .into_input()
        .into_pushpull()
        .into_pulldown();

    // unsafe {
    //     gpio.disable_clock();
    // }

    let syscfg = Syscfg::with_enabled_clock(periph_syscfg!(reg));
    let exti = ExtiDrv::new(periph_exti2!(reg), thr.exti2, &syscfg).into_rising_edge();

    pin.get();

    let line = exti.line(unsafe { pin.clone() });
    let mut stream = line.saturating_pulse_stream();

    while let Some(tick) = stream.next().root_wait() {
        let _ = pin.get();
    }

    // Enter the sleep state on ISR exit.
    core_reg
        .scb_scr
        .into_unsync()
        .modify(|r| r.set_sleeponexit());
}
