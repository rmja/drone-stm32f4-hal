//! The root task.

use crate::{consts, thr, thr::ThrsInit, Regs};
use drone_core::log;
use drone_cortexm::{reg::prelude::*, swo, thr::prelude::*};
use drone_stm32_map::periph::{
    dma::{periph_dma2, periph_dma2_ch2, periph_dma2_ch3},
    gpio::{
        periph_gpio_a5, periph_gpio_a6, periph_gpio_a7, periph_gpio_a_head, periph_gpio_b1,
        periph_gpio_b_head,
    },
    spi::periph_spi1,
};
use drone_stm32f4_hal::{
    dma::{config::*, DmaCfg},
    gpio::{GpioPinCfg, GpioPinSpeed},
    rcc::{periph_flash, periph_pwr, periph_rcc, traits::*, Flash, Pwr, Rcc, RccSetup},
    spi::{config::*, traits::*, SpiDrv, SpiIface},
};

/// The root task handler.
#[inline(never)]
pub fn handler(reg: Regs, thr_init: ThrsInit) {
    let thr = thr::init(thr_init);

    thr.hard_fault.add_once(|| panic!("Hard Fault"));

    println!("Hello, world!");

    // Enable interrupts.
    thr.rcc.enable_int();
    thr.spi_1.enable_int();
    thr.dma_2_ch_2.enable_int();
    thr.dma_2_ch_3.enable_int();

    // Enable IO port clock.
    let gpio_a = periph_gpio_a_head!(reg);
    let gpio_b = periph_gpio_b_head!(reg);
    gpio_a.rcc_busenr_gpioen.set_bit();
    gpio_b.rcc_busenr_gpioen.set_bit();

    // Configure SPI GPIO pins.
    GpioPinCfg::from(periph_gpio_a5!(reg)) // Clock.
        .into_af5()
        .into_pp()
        .with_speed(GpioPinSpeed::VeryHighSpeed);
    GpioPinCfg::from(periph_gpio_a6!(reg)) // MISO.
        .into_af5()
        .into_pp()
        .with_speed(GpioPinSpeed::VeryHighSpeed);
    GpioPinCfg::from(periph_gpio_a7!(reg)) // MOSI.
        .into_af5()
        .into_pp()
        .with_speed(GpioPinSpeed::VeryHighSpeed);
    let cs_pin = GpioPinCfg::from(periph_gpio_b1!(reg))
        .into_output()
        .with_speed(GpioPinSpeed::HighSpeed)
        .pin();

    // Disable IO port clock.
    gpio_a.rcc_busenr_gpioen.clear_bit();

    // Initialize clocks.
    let rcc_setup = RccSetup {
        rcc: periph_rcc!(reg),
        rcc_int: thr.rcc,
    };
    let rcc = Rcc::init(rcc_setup);
    let pwr = Pwr::init(periph_pwr!(reg));
    let flash = Flash::init(periph_flash!(reg));

    let hseclk = rcc.stabilize(consts::HSECLK).root_wait();
    let pll = rcc
        .select(consts::PLLSRC_HSECLK, hseclk)
        .stabilize(consts::PLL)
        .root_wait();
    let pclk1 = rcc.configure(consts::PCLK1);
    let pclk2 = rcc.configure(consts::PCLK2);
    pwr.enable_od();
    flash.set_latency(consts::HCLK.get_wait_states(VoltageRange::HighVoltage));
    swo::flush();
    swo::update_prescaler(consts::HCLK.f() / log::baud_rate!() - 1);
    rcc.select(consts::SYSCLK_PLL, pll.p());

    // Initialize dma.
    let dma2 = DmaCfg::init(periph_dma2!(reg));
    let miso_setup = DmaChSetup::init(periph_dma2_ch2!(reg), thr.dma_2_ch_2);
    let miso_dma = dma2.init_ch(miso_setup);
    let mosi_setup = DmaChSetup::init(periph_dma2_ch3!(reg), thr.dma_2_ch_3);
    let mosi_dma = dma2.init_ch(mosi_setup);

    // Initialize spi.
    let setup = SpiSetup::init(periph_spi1!(reg), thr.spi_1, pclk2, BaudRate::Max(10_000_000));
    let spi_drv = SpiDrv::init(setup);
    let mut spi_master = spi_drv.init_spi1_master(miso_dma, mosi_dma);

    // let iface = SpiIface::new(cs_pin);

    // spi_master.select(&iface);
    // let tx_buf = [1, 2, 3, 4].as_ref();
    // spi_master.write(tx_buf).root_wait();
    // spi_master.deselect(&iface);

    // Enter a sleep state on ISR exit.
    reg.scb_scr.sleeponexit.set_bit();
}
