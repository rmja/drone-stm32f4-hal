//! The root task.

use crate::{consts, thr, thr::ThrsInit, Regs};
use drone_core::log;
use drone_cortexm::{reg::prelude::*, swo, thr::prelude::*};
use drone_stm32_map::periph::gpio::*;
use drone_stm32f4_hal::{fmc::{FmcDrv, config::*}, gpio::{prelude::*, GpioHead}, rcc::{Flash, Pwr, Rcc, RccSetup, clktree::HClk, periph_flash, periph_pwr, periph_rcc, traits::*}};

/// The root task handler.
#[inline(never)]
pub fn handler(reg: Regs, thr_init: ThrsInit) {
    let thr = thr::init(thr_init);

    thr.hard_fault.add_once(|| panic!("Hard Fault"));

    println!("Hello, world!");

    // Enable interrupts.
    thr.rcc.enable_int();

    // Initialize clocks.
    let rcc = Rcc::init(RccSetup::new(periph_rcc!(reg), thr.rcc));
    let pwr = Pwr::init(periph_pwr!(reg));
    let flash = Flash::init(periph_flash!(reg));
    let hclk = setup_clktree(&rcc, &pwr, &flash).root_wait();

    let gpio_d = GpioHead::with_enabled_clock(periph_gpio_d_head!(reg));
    let gpio_e = GpioHead::with_enabled_clock(periph_gpio_e_head!(reg));
    let gpio_f = GpioHead::with_enabled_clock(periph_gpio_f_head!(reg));
    let gpio_g = GpioHead::with_enabled_clock(periph_gpio_g_head!(reg));
    let gpio_h = GpioHead::with_enabled_clock(periph_gpio_h_head!(reg));

    let sdram_pins = FmcSdRamPins::default()
        .sdclk(gpio_g.pin(periph_gpio_g8!(reg)).into_af())
        .sdcke1(gpio_h.pin(periph_gpio_h7!(reg)).into_af())
        .sdne1(gpio_h.pin(periph_gpio_h6!(reg)).into_af())
        .nras(gpio_f.pin(periph_gpio_f11!(reg)).into_af())
        .ncas(gpio_g.pin(periph_gpio_g15!(reg)).into_af())
        .sdnwe(gpio_h.pin(periph_gpio_h5!(reg)).into_af());

    let address_pins = FmcSdRamAddressPins::default()
        .a0(gpio_f.pin(periph_gpio_f0!(reg)).into_af())
        .a1(gpio_f.pin(periph_gpio_f1!(reg)).into_af())
        .a2(gpio_f.pin(periph_gpio_f2!(reg)).into_af())
        .a3(gpio_f.pin(periph_gpio_f3!(reg)).into_af())
        .a4(gpio_f.pin(periph_gpio_f4!(reg)).into_af())
        .a5(gpio_f.pin(periph_gpio_f5!(reg)).into_af())
        .a6(gpio_f.pin(periph_gpio_f12!(reg)).into_af())
        .a7(gpio_f.pin(periph_gpio_f13!(reg)).into_af())
        .a8(gpio_f.pin(periph_gpio_f14!(reg)).into_af())
        .a9(gpio_f.pin(periph_gpio_f15!(reg)).into_af())
        .a10(gpio_g.pin(periph_gpio_g0!(reg)).into_af())
        .a11(gpio_g.pin(periph_gpio_g1!(reg)).into_af());

    let data_pins = FmcSdRamDataPins::default()
        .d0(gpio_d.pin(periph_gpio_d14!(reg)).into_af())
        .d1(gpio_d.pin(periph_gpio_d15!(reg)).into_af())
        .d2(gpio_d.pin(periph_gpio_d0!(reg)).into_af())
        .d3(gpio_d.pin(periph_gpio_d1!(reg)).into_af())
        .d4(gpio_e.pin(periph_gpio_e7!(reg)).into_af())
        .d5(gpio_e.pin(periph_gpio_e8!(reg)).into_af())
        .d6(gpio_e.pin(periph_gpio_e9!(reg)).into_af())
        .d7(gpio_e.pin(periph_gpio_e10!(reg)).into_af())
        .d8(gpio_e.pin(periph_gpio_e11!(reg)).into_af())
        .d9(gpio_e.pin(periph_gpio_e12!(reg)).into_af())
        .d10(gpio_e.pin(periph_gpio_e13!(reg)).into_af())
        .d11(gpio_e.pin(periph_gpio_e14!(reg)).into_af())
        .d12(gpio_e.pin(periph_gpio_e15!(reg)).into_af())
        .d13(gpio_d.pin(periph_gpio_d8!(reg)).into_af())
        .d14(gpio_d.pin(periph_gpio_d9!(reg)).into_af())
        .d15(gpio_d.pin(periph_gpio_d10!(reg)).into_af());

    let bank_pins = FmcSdRamBankPins::default()
        .ba0(gpio_g.pin(periph_gpio_g4!(reg)).into_af())
        .ba1(gpio_g.pin(periph_gpio_g5!(reg)).into_af());

    let mask_pins = FmcSdRamByteMaskPins::default()
        .nbl0(gpio_e.pin(periph_gpio_e0!(reg)).into_af())
        .nbl1(gpio_e.pin(periph_gpio_e1!(reg)).into_af());

    FmcDrv::init_sdram(SdRamSetup::new(consts::SDRAM_BANK, consts::SDRAM_CFG, hclk), sdram_pins, address_pins, data_pins, bank_pins, mask_pins);

    // Enter a sleep state on ISR exit.
    reg.scb_scr.sleeponexit.set_bit();
}

async fn setup_clktree(rcc: &Rcc<thr::Rcc>, pwr: &Pwr, flash: &Flash) -> ConfiguredClk<HClk> {
    let hseclk = rcc.stabilize(consts::HSECLK).await;
    let pll = rcc
        .select(consts::PLLSRC_HSECLK, hseclk)
        .stabilize(consts::PLL)
        .await;
    // let pclk1 = rcc.configure(consts::PCLK1);
    // let pclk2 = rcc.configure(consts::PCLK2);
    pwr.enable_overdrive();
    flash.set_latency(consts::HCLK.get_wait_states(VoltageRange::HighVoltage));
    swo::flush();
    swo::update_prescaler(consts::HCLK.f() / log::baud_rate!() - 1);
    rcc.select(consts::SYSCLK_PLL, pll.p());
    let hclk = rcc.configure(consts::HCLK);
    hclk
}