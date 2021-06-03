//! The root task.

use crate::{consts, thr, thr::ThrsInit, Regs};
use drone_core::log;
use drone_cortexm::{reg::prelude::*, swo, thr::prelude::*};
use drone_stm32_map::periph::{
    dma::{periph_dma2, periph_dma2_ch2, periph_dma2_ch3},
    gpio::{
        periph_gpio_a5, periph_gpio_a6, periph_gpio_a7, periph_gpio_a_head, periph_gpio_b7,
        periph_gpio_b_head,
    },
    spi::periph_spi1,
};
use drone_stm32f4_hal::{
    dma::{config::*, DmaCfg},
    gpio::{prelude::*, GpioHead},
    rcc::{prelude::*, periph_flash, periph_pwr, periph_rcc, Flash, Pwr, Rcc, RccSetup},
    spi::{prelude::*, chipctrl::*, SpiDrv, SpiSetup, SpiPins},
};

/// The root task handler.
#[inline(never)]
pub fn handler(reg: Regs, thr_init: ThrsInit) {
    let thr = thr::init(thr_init);

    thr.hard_fault.add_once(|| panic!("Hard Fault"));

    println!("Hello, world!");

    // Enable interrupts.
    thr.rcc.enable_int();
    thr.spi1.enable_int();
    thr.dma2_ch2.enable_int();
    thr.dma2_ch3.enable_int();

    // Enable IO port clock.
    let gpio_a = GpioHead::with_enabled_clock(periph_gpio_a_head!(reg));
    let gpio_b = GpioHead::with_enabled_clock(periph_gpio_b_head!(reg));

    // Configure SPI GPIO pins.
    let pin_sck = gpio_a.pin(periph_gpio_a5!(reg))
        .into_alternate()
        .into_pushpull()
        .with_speed(GpioPinSpeed::MediumSpeed);
    let pin_miso = gpio_a.pin(periph_gpio_a6!(reg))
        .into_alternate()
        .into_pushpull()
        .with_speed(GpioPinSpeed::MediumSpeed);
    let pin_mosi = gpio_a.pin(periph_gpio_a7!(reg))
        .into_alternate()
        .into_pushpull()
        .with_speed(GpioPinSpeed::MediumSpeed);
    let pin_cs = gpio_b.pin(periph_gpio_b7!(reg))
        .into_output()
        .into_pushpull()
        .into_pullup()
        .with_speed(GpioPinSpeed::MediumSpeed);

    // Disable IO port clock.
    unsafe {
        gpio_a.disable_clock();
    }

    // Initialize clocks.
    let rcc = Rcc::init(RccSetup::new(periph_rcc!(reg), thr.rcc));
    let pwr = Pwr::with_enabled_clock(periph_pwr!(reg));
    let flash = Flash::new(periph_flash!(reg));

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

    // Initialize dma.
    let dma2 = DmaCfg::with_enabled_clock(periph_dma2!(reg));
    let miso_dma = dma2.ch(DmaChSetup::new(periph_dma2_ch2!(reg), thr.dma2_ch2));
    let mosi_dma = dma2.ch(DmaChSetup::new(periph_dma2_ch3!(reg), thr.dma2_ch3));

    // Initialize spi.
    let pins = SpiPins::default().sck(pin_sck).miso(pin_miso).mosi(pin_mosi);
    let setup = SpiSetup::new(
        periph_spi1!(reg),
        thr.spi1,
        pins,
        pclk2,
        BaudRate::Max(7_700_000),
    );
    let mut spi = SpiDrv::init(setup).into_master(miso_dma, mosi_dma);

    let mut chip = SpiChip::new_deselected(pin_cs);

    loop {
        let selection = spi.select(&mut chip);
        let tx_buf = [1, 2, 3, 4].as_ref();
        let mut rx_buf = [0;4];
        spi.write(tx_buf).root_wait();
        spi.read(&mut rx_buf).root_wait();
        spi.xfer(tx_buf, &mut rx_buf).root_wait();
        drop(selection); // drop() deselects chip.
    }

    // Enter a sleep state on ISR exit.
    reg.scb_scr.sleeponexit.set_bit();
}
