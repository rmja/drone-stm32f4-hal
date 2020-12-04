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
        periph_gpio_a2,
        periph_gpio_a3,
        // periph_gpio_c_head, periph_gpio_c10, periph_gpio_c11,
        periph_gpio_a_head,
        periph_gpio_b10,
        periph_gpio_b2,
        periph_gpio_b_head,
    },
    uart::{periph_usart2, periph_usart3},
};
use drone_stm32f4_hal::{
    gpio::{GpioPinCfg, GpioPinSpeed},
    rcc::RccSetup,
    uart::config::DataBits,
    uart::{
        config::{Parity, StopBits, UartClk, UartDmaSetup, UartSetup},
        UartDrv,
    },
};

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

    let setup = UartSetup::default(periph_usart2!(reg), thr.usart_2).at(
        9_600,
        DataBits::Eight,
        Parity::None,
        StopBits::One,
    );
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

    let adapters = Adapters {};
    let uart_drv = UartDrv::init(setup, adapters);
    let mut tx_drv = uart_drv.tx(tx_setup);

    let rx_ring_buf = vec![0; 10].into_boxed_slice();
    let mut rx_drv = uart_drv.rx(rx_setup);

    // Enable receiver.
    let mut rx = rx_drv.sess(rx_ring_buf);

    {
        let mut tx = tx_drv.sess();
        tx.write(b"Write a lowercase word:\n").root_wait();
        tx.flush().root_wait();
    }


    let mut line_buf = vec![];

    loop {
        let mut buf = [0; 4];
        match rx.read(&mut buf).root_wait() {
            Ok(n) => {
                line_buf.extend_from_slice(&buf[..n]);
            },
            Err(e) => {
                line_buf.clear();
                line_buf.extend_from_slice(format!("Error: {:?}\n", e).as_bytes());
            }
        };
        
        let newline = line_buf.iter().position(|x| { x == &b'\n'});
        let line = match newline {
            Some(index) => &line_buf[..index],
            None => continue,
        };

        // Write back the uppercase equivalent of the received.
        let mut upper = String::from_utf8(line.to_vec()).unwrap_or(String::from("?"));
        upper.make_ascii_uppercase();

        // The calls to write() finishes as soon as the tx session can receive more bytes,
        // and not when when transmission has actually completed.
        // This enables full saturation of the uart.

        // Enable transmitter.
        let mut tx = tx_drv.sess();

        dbg1.set();
        tx.write(upper.into_bytes().as_ref()).root_wait();
        dbg1.clear();
        tx.write(b"\n").root_wait();
        dbg1.set();
        tx.flush().root_wait(); // Wait for the actual uart transmission to complete
        dbg1.clear();

        // Dropping tx disables the transmitter.
        // This is a busy wait if flush() is not called prior to dropping tx!
        drop(tx);

        line_buf.clear();
    }

    // Enter a sleep state on ISR exit.
    reg.scb_scr.sleeponexit.set_bit();
}
