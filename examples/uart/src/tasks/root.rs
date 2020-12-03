//! The root task.

use crate::{thr, thr::ThrsInit, Regs};
use drone_core::log;
use drone_cortexm::{reg::prelude::*, swo, thr::prelude::*};
use drone_stm32_map::periph::{
    dma::{
        periph_dma1,
        periph_dma1_ch5,
        periph_dma1_ch6,
        // periph_dma1_ch1,
        // periph_dma1_ch3,
    },
    gpio::{
        periph_gpio_b10, periph_gpio_b2, periph_gpio_b_head,
        periph_gpio_a_head, periph_gpio_a2, periph_gpio_a3,
        // periph_gpio_c_head, periph_gpio_c10, periph_gpio_c11,
    },
    uart::{periph_usart2, periph_usart3},
};
use drone_stm32f4_hal::{gpio::{GpioPinCfg, GpioPinSpeed}, rcc::RccSetup, uart::config::DataBits, uart::{UartDrv, config::{Parity, StopBits, UartClk, UartDmaSetup, UartSetup}}};

struct Adapters;

impl UartClk for Adapters {
    fn clock(&self) -> u32 {
        90_000_000
    }
}

/// The root task handler.
#[inline(never)]
pub fn handler(reg: Regs, thr_init: ThrsInit) {
    let thr = thr::init(thr_init);

    thr.hard_fault.add_once(|| panic!("Hard Fault"));

    println!("Hello, world!");

    // Enable interrupts.
    thr.rcc.enable_int();
    thr.usart_2.enable_int();
    thr.dma_1_ch_5.enable_int();
    thr.dma_1_ch_6.enable_int();
    // thr.usart_3.enable_int();
    // thr.dma_1_ch_1.enable_int();
    // thr.dma_1_ch_3.enable_int();

    // Enable IO port clock.
    let gpio_a = periph_gpio_a_head!(reg);
    gpio_a.rcc_busenr_gpioen.set_bit();
    // let gpio_c = periph_gpio_c_head!(reg);
    // gpio_c.rcc_busenr_gpioen.set_bit();

    // Configure UART GPIO pins.
    GpioPinCfg::from(periph_gpio_a2!(reg))
        .into_af7()
        .into_pp()
        .with_speed(GpioPinSpeed::VeryHighSpeed);
    GpioPinCfg::from(periph_gpio_a3!(reg))
        .into_af7()
        .into_pp()
        .with_speed(GpioPinSpeed::VeryHighSpeed);
    // GpioPinCfg::from(periph_gpio_c10!(reg))
    //     .into_af7()
    //     .into_pp()
    //     .with_speed(GpioPinSpeed::VeryHighSpeed);
    // GpioPinCfg::from(periph_gpio_c11!(reg))
    //     .into_af7()
    //     .into_pp()
    //     .with_speed(GpioPinSpeed::VeryHighSpeed);

    // Disable IO port clock.
    gpio_a.rcc_busenr_gpioen.clear_bit();
    // gpio_c.rcc_busenr_gpioen.clear_bit();

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

    let setup = UartSetup::default(periph_usart2!(reg), thr.usart_2)
        .at(9_600, DataBits::Eight, Parity::None, StopBits::One);
    let tx_setup = UartDmaSetup {
        dma: periph_dma1_ch6!(reg),
        dma_int: thr.dma_1_ch_6,
        dma_ch: 4,
        dma_pl: 1, // Priority level: medium
    };
    let rx_setup = UartDmaSetup {
        dma: periph_dma1_ch5!(reg),
        dma_int: thr.dma_1_ch_5,
        dma_ch: 4,
        dma_pl: 1, // Priority level: medium
    };

    // let setup = UartSetup::default(periph_usart3!(reg), thr.usart_3);
    // let tx_setup = UartDmaSetup {
    //     dma: periph_dma1_ch3!(reg),
    //     dma_int: thr.dma_1_ch_3,
    //     dma_ch: 4,
    //     dma_pl: 1, // Priority level: medium
    // };
    // let rx_setup = UartDmaSetup {
    //     dma: periph_dma1_ch1!(reg),
    //     dma_int: thr.dma_1_ch_1,
    //     dma_ch: 4,
    //     dma_pl: 1, // Priority level: medium
    // };

    let adapters = Adapters{};
    let uart_drv = UartDrv::init(setup, adapters);
    let mut tx_drv = uart_drv.tx(tx_setup);


    // let rx_buf = Vec::with_capacity(128).into_boxed_slice();
    // let mut rx = uart_drv.rx(rx_buf);
    // let mut buf = [0u8, 1];
    // rx.read(&mut buf);

    loop {
        let mut tx = tx_drv.sess();
        // let writebuf = [0x55, 0xAA, 0x55, 0xAA].as_ref();
        let writebuf = b"Hello".as_ref();
        let writebuf2 = b"World\r\n".as_ref();
        dbg1.set();
        tx.write(writebuf).root_wait();
        dbg1.clear();
        tx.write(writebuf2).root_wait();
        dbg1.set();
        tx.flush().root_wait();
        dbg1.clear();
        tx.write(b"Drone OS is awesome!\r\n".as_ref()).root_wait();
        dbg1.set();
        drop(tx); // Dropping tx guard disables TX on the uart - this is a busy wait if flush() is not called prior to drop
        dbg1.clear();
    }

    // Enter a sleep state on ISR exit.
    reg.scb_scr.sleeponexit.set_bit();
}
