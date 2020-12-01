//! The root task.

use crate::{thr, thr::ThrsInit, Regs};
use drone_core::log;
use drone_cortexm::{reg::prelude::*, swo, thr::prelude::*};
use drone_stm32_map::{
    periph::{
        gpio::{
            periph_gpio_b2,
            periph_gpio_b10,
            periph_gpio_b_head,
            periph_gpio_c_head,
            periph_gpio_c10,
            periph_gpio_c11,
        },
        dma::{periph_dma1, periph_dma1_ch3},
        uart::periph_usart3,
    }
};
use drone_stm32f4_hal:: {rcc::RccSetup, gpio::{GpioPinCfg, GpioPinSpeed}, uart::{UartDrv, UartParity, UartSetup, UartStop}};

/// The root task handler.
#[inline(never)]
pub fn handler(reg: Regs, thr_init: ThrsInit) {
    let thr = thr::init(thr_init);

    thr.hard_fault.add_once(|| panic!("Hard Fault"));

    println!("Hello, world!");

    // Enable interrupts.
    thr.rcc.enable_int();
    thr.dma_1_ch_3.enable_int();
    thr.usart_3.enable_int();

    // Enable IO port clock.
    let gpio_c = periph_gpio_c_head!(reg);
    gpio_c.rcc_busenr_gpioen.set_bit();

    // Configure UART GPIO pins.
    GpioPinCfg::from(periph_gpio_c10!(reg))
        .into_af7_usart_1_2_3()
        .into_pp()
        .with_speed(GpioPinSpeed::VeryHighSpeed);

    GpioPinCfg::from(periph_gpio_c11!(reg))
        .into_af7_usart_1_2_3()
        .into_pp()
        .with_speed(GpioPinSpeed::VeryHighSpeed);

    // Disable IO port clock.
    gpio_c.rcc_busenr_gpioen.clear_bit();

    // Configure debug pins used for capturing logic analyzer shots.
    let gpio_b = periph_gpio_b_head!(reg);
    gpio_b.rcc_busenr_gpioen.set_bit(); // Enable IO port clock

    let mut dbg1 = GpioPinCfg::from(periph_gpio_b2!(reg))
        .into_output()
        .into_pp()
        .with_speed(GpioPinSpeed::VeryHighSpeed);

    // let mut dbg2 = GpioPinCfg::from(periph_gpio_b10!(reg))
    //     .into_output()
    //     .into_pp()
    //     .with_speed(GpioPinSpeed::VeryHighSpeed);

    // Enable DMA clock.
    let dma1 = periph_dma1!(reg);
    dma1.rcc_busenr_dmaen.set_bit();




    let rcc = RccSetup {
        rcc_cr: reg.rcc_cr,
        rcc_pllcfgr: reg.rcc_pllcfgr,
        rcc_cfgr: reg.rcc_cfgr,
        rcc_cir: reg.rcc_cir,

        flash_acr: reg.flash_acr,
        pwr_cr: reg.pwr_cr,
        pwr_csr: reg.pwr_csr,
        thr_rcc: thr.rcc,
    };

    reg.rcc_apb1enr.modify(|r| r.set_pwren());

    swo::flush();
    rcc.apply().root_wait();

    swo::update_prescaler(180_000_000 / log::baud_rate!() - 1);


    let setup = UartSetup {
        uart: periph_usart3!(reg),
        uart_int: thr.usart_3,
        uart_baud_rate: 9600,
        uart_word_length: 8,
        uart_stop_bits: UartStop::One,
        uart_parity: UartParity::None,
        uart_oversampling: 16,
        dma_tx: periph_dma1_ch3!(reg),
        dma_tx_int: thr.dma_1_ch_3,
        dma_tx_ch: 4,
        dma_tx_pl: 1, // Priority level: medium
    };

    let uart_drv = UartDrv::init(setup);

    loop {
        let mut tx = uart_drv.tx();

        let writebuf = [0x55, 0xAA, 0x55, 0xAA].as_ref();
        let writebuf2 = [0x33, 0xEE, 0x33, 0xEE].as_ref();
        dbg1.set();
        tx.write(writebuf).root_wait();
        dbg1.clear();
        tx.write(writebuf2).root_wait();
        tx.write(writebuf).root_wait();
        dbg1.set();
        tx.flush().root_wait();
        dbg1.clear();
        tx.write([0xFF, 0x77, 0x33, 0x11].as_ref()).root_wait();
        dbg1.set();
        drop(tx); // Dropping tx guard disables TX on the uart - this is a busy wait if flush() is not called prior to drop
        dbg1.clear();
    }

    // Enter a sleep state on ISR exit.
    reg.scb_scr.sleeponexit.set_bit();
}
