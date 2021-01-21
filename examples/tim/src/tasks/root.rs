//! The root task.

use crate::{consts, thr, thr::ThrsInit, Regs};
use drone_core::log;
use drone_cortexm::{reg::prelude::*, swo, thr::prelude::*};
use drone_stm32_map::periph::{
    gpio::{periph_gpio_d13, periph_gpio_d_head},
    tim::periph_tim2,
    tim::periph_tim4,
};
use drone_stm32f4_hal::{
    gpio::{prelude::*, GpioHead, GpioPinSpeed},
    rcc::{periph_flash, periph_pwr, periph_rcc, traits::*, Flash, Pwr, Rcc, RccSetup},
    tim::{prelude::*, GeneralTimCfg, GeneralTimSetup},
};
use futures::prelude::*;

/// The root task handler.
#[inline(never)]
pub fn handler(reg: Regs, thr_init: ThrsInit) {
    let thr = thr::init(thr_init);

    thr.hard_fault.add_once(|| panic!("Hard Fault"));

    println!("Hello, world!");

    // Enable interrupts.
    thr.rcc.enable_int();
    thr.tim_2.enable_int();
    thr.tim_4.enable_int();

    // Initialize clocks.
    let rcc = Rcc::init(RccSetup::new(periph_rcc!(reg), thr.rcc));
    let pwr = Pwr::init(periph_pwr!(reg));
    let flash = Flash::init(periph_flash!(reg));

    let hseclk = rcc.stabilize(consts::HSECLK).root_wait();
    let pll = rcc
        .select(consts::PLLSRC_HSECLK, hseclk)
        .stabilize(consts::PLL)
        .root_wait();
    let hclk = rcc.configure(consts::HCLK);
    let pclk1 = rcc.configure(consts::PCLK1);
    let pclk2 = rcc.configure(consts::PCLK2);
    pwr.enable_overdrive();
    flash.set_latency(consts::HCLK.get_wait_states(VoltageRange::HighVoltage));
    swo::flush();
    swo::update_prescaler(consts::HCLK.f() / log::baud_rate!() - 1);
    rcc.select(consts::SYSCLK_PLL, pll.p());

    // Configure timer.
    let tim2_setup = GeneralTimSetup::new(
        periph_tim2!(reg),
        thr.tim_2,
        pclk1,
        TimFreq::Nominal(consts::TIM2_FREQ),
    );
    let tim4_setup = GeneralTimSetup::new(
        periph_tim4!(reg),
        thr.tim_4,
        pclk1,
        TimFreq::Nominal(consts::TIM2_FREQ),
    );

    // Enable IO port clock.
    let gpio_d = GpioHead::with_enabled_clock(periph_gpio_d_head!(reg));

    let capture_pin = gpio_d
        .pin(periph_gpio_d13!(reg))
        .into_af()
        .into_pp()
        .into_pulldown()
        .with_speed(GpioPinSpeed::MediumSpeed);

    let tim2 = GeneralTimCfg::with_enabled_clock(tim2_setup)
        .into_count_up()
        .into_master();

    let mut tim4 = GeneralTimCfg::with_enabled_clock(tim4_setup)
        .into_count_up()
        .ch1(|ch| ch.into_input_capture_pin(capture_pin))
        .ch2(|ch| ch.into_output_compare())
        .into_trigger_slave_of(tim2.link);

    tim2.start();

    // let mut overflow_stream = tim4.ovf.saturating_pulse_stream();
    let mut capture_stream = tim4.ch1.saturating_stream(10);
    while let Some(capture) = capture_stream.next().root_wait() {
        println!(
            "TIM2 counter: {}, TIM4 counter: {}, TIM4 capture: {}",
            tim2.cnt.value(),
            tim4.cnt.value(),
            capture
        );
        swo::flush();
    }

    // Enter a sleep state on ISR exit.
    reg.scb_scr.sleeponexit.set_bit();
}
