//! The root task.

use crate::{consts, thr, thr::ThrsInit, Regs};
use drone_core::log;
use drone_cortexm::{reg::prelude::*, swo, thr::prelude::*};
use drone_stm32_map::periph::gpio::*;
use drone_stm32f4_hal::{
    fmc::{sdram_pins, config::*, periph_fmc, FmcDrv},
    gpio::{prelude::*, GpioHead},
    rcc::{
        clktree::HClk, periph_flash, periph_pwr, periph_rcc, traits::*, Flash, Pwr, Rcc, RccSetup,
    },
};

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

    // Configure pins.
    let gpio_d = GpioHead::with_enabled_clock(periph_gpio_d_head!(reg));
    let gpio_e = GpioHead::with_enabled_clock(periph_gpio_e_head!(reg));
    let gpio_f = GpioHead::with_enabled_clock(periph_gpio_f_head!(reg));
    let gpio_g = GpioHead::with_enabled_clock(periph_gpio_g_head!(reg));
    let gpio_h = GpioHead::with_enabled_clock(periph_gpio_h_head!(reg));

    let sdram_pins = sdram_pins!(FmcSdRamPins, reg,
        gpio_f => sdnras;
        gpio_g => sdclk, sdncas;
        gpio_h => sdcke1, sdne1, sdnwe;
    );

    let address_pins = sdram_pins!(FmcSdRamAddressPins, reg,
        gpio_f => a0, a1, a2, a3, a4, a5, a6, a7, a8, a9;
        gpio_g => a10, a11;
    );

    let data_pins = sdram_pins!(FmcSdRamDataPins, reg,
        gpio_d => d0, d1, d2, d3;
        gpio_e => d4, d5, d6, d7, d8, d9, d10, d11, d12;
        gpio_d => d13, d14, d15;
    );

    let bank_pins = sdram_pins!(FmcSdRamBankPins, reg,
        gpio_g => ba0, ba1;
    );

    let mask_pins = sdram_pins!(FmcSdRamByteMaskPins, reg,
        gpio_e => nbl0, nbl1;
    );

    let fmc = FmcDrv::init_sdram(
        SdRamSetup::for_bank2(periph_fmc!(reg), consts::SDRAM_CFG, hclk),
        sdram_pins,
        address_pins,
        data_pins,
        bank_pins,
        mask_pins,
    );

    // IMPORTANT: Do not run the sanity check sequence when using the heap!
    if true {
        let ram = fmc.bank2_slice::<usize>();

        for i in 0..ram.len() {
            ram[i] = i;
        }

        for i in 0..ram.len() {
            assert_eq!(i, ram[i], "SDRAM sanity check error!");
        }
    }
    else {
        use crate::{HEAP, HEAP_SLOW};
        // This does not yet work due to an rust compiler error, see https://github.com/rust-lang/rust/issues/78459
        // let y = Box::new_in((), &HEAP_SLOW);
    }

    // Enter a sleep state on ISR exit.
    reg.scb_scr.sleeponexit.set_bit();
}

async fn setup_clktree(rcc: &Rcc<thr::Rcc>, pwr: &Pwr, flash: &Flash) -> ConfiguredClk<HClk> {
    let hseclk = rcc.stabilize(consts::HSECLK).await;
    let pll = rcc
        .select(consts::PLLSRC_HSECLK, hseclk)
        .stabilize(consts::PLL)
        .await;
    let hclk = rcc.configure(consts::HCLK);
    let pclk1 = rcc.configure(consts::PCLK1);
    let pclk2 = rcc.configure(consts::PCLK2);
    pwr.enable_overdrive();
    flash.set_latency(consts::HCLK.get_wait_states(VoltageRange::HighVoltage));
    swo::flush();
    swo::update_prescaler(consts::HCLK.f() / log::baud_rate!() - 1);
    rcc.select(consts::SYSCLK_PLL, pll.p());
    hclk
}
