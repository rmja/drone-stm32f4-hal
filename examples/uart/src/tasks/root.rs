//! The root task.

use crate::{consts, thr, thr::ThrsInit, Regs};
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
    dma::{DmaCfg, DmaChSetup},
    gpio::{GpioPinCfg, GpioPinSpeed},
    rcc::{periph_flash, periph_pwr, periph_rcc, traits::*, Flash, Pwr, Rcc, RccSetup},
    uart::{config::*, UartDrv},
};

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

    // Enable IO port clock.
    let gpio_a = periph_gpio_a_head!(reg);
    gpio_a.rcc_busenr_gpioen.set_bit();
    // let gpio_c = periph_gpio_c_head!(reg);
    // gpio_c.rcc_busenr_gpioen.set_bit();

    // Configure UART GPIO pins.
    GpioPinCfg::from(periph_gpio_a2!(reg)) // TX.
        .into_af7()
        .into_pp()
        .with_speed(GpioPinSpeed::VeryHighSpeed);
    GpioPinCfg::from(periph_gpio_a3!(reg)) // RX.
        .into_af7()
        .into_pp()
        .with_speed(GpioPinSpeed::VeryHighSpeed);

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

    let setup = UartSetup::usart2(periph_usart2!(reg), thr.usart_2, pclk1);

    let dma1 = DmaCfg::init(periph_dma1!(reg));
    let rx_setup = DmaChSetup::dma1_ch5_stch4(periph_dma1_ch5!(reg), thr.dma_1_ch_5);
    let rx_dma = dma1.init_ch(rx_setup);
    let tx_setup = DmaChSetup::dma1_ch6_stch4(periph_dma1_ch6!(reg), thr.dma_1_ch_6);
    let tx_dma = dma1.init_ch(tx_setup);

    let uart_drv = UartDrv::init(setup);
    let mut rx_drv = uart_drv.init_usart1_rx(rx_dma);
    let mut tx_drv = uart_drv.init_usart1_tx(tx_dma);

    // Enable receiver.
    let rx_ring_buf = vec![0; 10].into_boxed_slice();
    let mut rx = rx_drv.start(rx_ring_buf);

    {
        let mut tx = tx_drv.start();
        tx.write(b"Write a lowercase word:\n").root_wait();
        tx.flush().root_wait();
    }

    let mut line_buf = vec![];

    loop {
        let mut buf = [0; 4];
        match rx.read(&mut buf).root_wait() {
            Ok(n) => {
                line_buf.extend_from_slice(&buf[..n]);
            }
            Err(e) => {
                line_buf.clear();
                line_buf.extend_from_slice(format!("Error: {:?}\n", e).as_bytes());
            }
        };

        let newline = line_buf.iter().position(|x| x == &b'\n');
        let line = match newline {
            Some(index) => &line_buf[..index],
            None => continue,
        };

        // Write back the uppercase equivalent of the received.
        let mut upper = String::from_utf8(line.to_vec()).unwrap_or_else(|_| String::from("?"));
        upper.make_ascii_uppercase();

        // The calls to write() finishes as soon as the tx session can receive more bytes,
        // and not when when transmission has actually completed.
        // This enables full saturation of the uart.

        // Enable transmitter.
        let mut tx = tx_drv.start();

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
