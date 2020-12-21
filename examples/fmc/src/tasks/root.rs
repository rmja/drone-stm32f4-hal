//! The root task.

use crate::{consts, thr, thr::ThrsInit, Regs};
use drone_core::log;
use drone_cortexm::{reg::prelude::*, swo, thr::prelude::*};
use drone_stm32_map::periph::gpio::*;
use drone_stm32f4_hal::{
    fmc::{config::*, periph_fmc, FmcDrv},
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

    let gpio_d = GpioHead::with_enabled_clock(periph_gpio_d_head!(reg));
    let gpio_e = GpioHead::with_enabled_clock(periph_gpio_e_head!(reg));
    let gpio_f = GpioHead::with_enabled_clock(periph_gpio_f_head!(reg));
    let gpio_g = GpioHead::with_enabled_clock(periph_gpio_g_head!(reg));
    let gpio_h = GpioHead::with_enabled_clock(periph_gpio_h_head!(reg));

    let sdram_pins = FmcSdRamPins::default()
        .sdclk(
            gpio_g
                .pin(periph_gpio_g8!(reg))
                .into_af()
                .with_speed(GpioPinSpeed::HighSpeed),
        )
        .sdcke1(
            gpio_h
                .pin(periph_gpio_h7!(reg))
                .into_af()
                .with_speed(GpioPinSpeed::HighSpeed),
        )
        .sdne1(
            gpio_h
                .pin(periph_gpio_h6!(reg))
                .into_af()
                .with_speed(GpioPinSpeed::HighSpeed),
        )
        .nras(
            gpio_f
                .pin(periph_gpio_f11!(reg))
                .into_af()
                .with_speed(GpioPinSpeed::HighSpeed),
        )
        .ncas(
            gpio_g
                .pin(periph_gpio_g15!(reg))
                .into_af()
                .with_speed(GpioPinSpeed::HighSpeed),
        )
        .sdnwe(
            gpio_h
                .pin(periph_gpio_h5!(reg))
                .into_af()
                .with_speed(GpioPinSpeed::HighSpeed),
        );

    let address_pins = FmcSdRamAddressPins::default()
        .a0(gpio_f
            .pin(periph_gpio_f0!(reg))
            .into_af()
            .with_speed(GpioPinSpeed::HighSpeed))
        .a1(gpio_f
            .pin(periph_gpio_f1!(reg))
            .into_af()
            .with_speed(GpioPinSpeed::HighSpeed))
        .a2(gpio_f
            .pin(periph_gpio_f2!(reg))
            .into_af()
            .with_speed(GpioPinSpeed::HighSpeed))
        .a3(gpio_f
            .pin(periph_gpio_f3!(reg))
            .into_af()
            .with_speed(GpioPinSpeed::HighSpeed))
        .a4(gpio_f
            .pin(periph_gpio_f4!(reg))
            .into_af()
            .with_speed(GpioPinSpeed::HighSpeed))
        .a5(gpio_f
            .pin(periph_gpio_f5!(reg))
            .into_af()
            .with_speed(GpioPinSpeed::HighSpeed))
        .a6(gpio_f
            .pin(periph_gpio_f12!(reg))
            .into_af()
            .with_speed(GpioPinSpeed::HighSpeed))
        .a7(gpio_f
            .pin(periph_gpio_f13!(reg))
            .into_af()
            .with_speed(GpioPinSpeed::HighSpeed))
        .a8(gpio_f
            .pin(periph_gpio_f14!(reg))
            .into_af()
            .with_speed(GpioPinSpeed::HighSpeed))
        .a9(gpio_f
            .pin(periph_gpio_f15!(reg))
            .into_af()
            .with_speed(GpioPinSpeed::HighSpeed))
        .a10(
            gpio_g
                .pin(periph_gpio_g0!(reg))
                .into_af()
                .with_speed(GpioPinSpeed::HighSpeed),
        )
        .a11(
            gpio_g
                .pin(periph_gpio_g1!(reg))
                .into_af()
                .with_speed(GpioPinSpeed::HighSpeed),
        );

    let data_pins = FmcSdRamDataPins::default()
        .d0(gpio_d
            .pin(periph_gpio_d14!(reg))
            .into_af()
            .with_speed(GpioPinSpeed::HighSpeed))
        .d1(gpio_d
            .pin(periph_gpio_d15!(reg))
            .into_af()
            .with_speed(GpioPinSpeed::HighSpeed))
        .d2(gpio_d
            .pin(periph_gpio_d0!(reg))
            .into_af()
            .with_speed(GpioPinSpeed::HighSpeed))
        .d3(gpio_d
            .pin(periph_gpio_d1!(reg))
            .into_af()
            .with_speed(GpioPinSpeed::HighSpeed))
        .d4(gpio_e
            .pin(periph_gpio_e7!(reg))
            .into_af()
            .with_speed(GpioPinSpeed::HighSpeed))
        .d5(gpio_e
            .pin(periph_gpio_e8!(reg))
            .into_af()
            .with_speed(GpioPinSpeed::HighSpeed))
        .d6(gpio_e
            .pin(periph_gpio_e9!(reg))
            .into_af()
            .with_speed(GpioPinSpeed::HighSpeed))
        .d7(gpio_e
            .pin(periph_gpio_e10!(reg))
            .into_af()
            .with_speed(GpioPinSpeed::HighSpeed))
        .d8(gpio_e
            .pin(periph_gpio_e11!(reg))
            .into_af()
            .with_speed(GpioPinSpeed::HighSpeed))
        .d9(gpio_e
            .pin(periph_gpio_e12!(reg))
            .into_af()
            .with_speed(GpioPinSpeed::HighSpeed))
        .d10(
            gpio_e
                .pin(periph_gpio_e13!(reg))
                .into_af()
                .with_speed(GpioPinSpeed::HighSpeed),
        )
        .d11(
            gpio_e
                .pin(periph_gpio_e14!(reg))
                .into_af()
                .with_speed(GpioPinSpeed::HighSpeed),
        )
        .d12(
            gpio_e
                .pin(periph_gpio_e15!(reg))
                .into_af()
                .with_speed(GpioPinSpeed::HighSpeed),
        )
        .d13(
            gpio_d
                .pin(periph_gpio_d8!(reg))
                .into_af()
                .with_speed(GpioPinSpeed::HighSpeed),
        )
        .d14(
            gpio_d
                .pin(periph_gpio_d9!(reg))
                .into_af()
                .with_speed(GpioPinSpeed::HighSpeed),
        )
        .d15(
            gpio_d
                .pin(periph_gpio_d10!(reg))
                .into_af()
                .with_speed(GpioPinSpeed::HighSpeed),
        );

    let bank_pins = FmcSdRamBankPins::default()
        .ba0(
            gpio_g
                .pin(periph_gpio_g4!(reg))
                .into_af()
                .with_speed(GpioPinSpeed::HighSpeed),
        )
        .ba1(
            gpio_g
                .pin(periph_gpio_g5!(reg))
                .into_af()
                .with_speed(GpioPinSpeed::HighSpeed),
        );

    let mask_pins = FmcSdRamByteMaskPins::default()
        .nbl0(
            gpio_e
                .pin(periph_gpio_e0!(reg))
                .into_af()
                .with_speed(GpioPinSpeed::HighSpeed),
        )
        .nbl1(
            gpio_e
                .pin(periph_gpio_e1!(reg))
                .into_af()
                .with_speed(GpioPinSpeed::HighSpeed),
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
