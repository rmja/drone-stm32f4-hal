//! The root task.

use crate::{consts, thr, thr::ThrsInit, Regs};
use drone_core::log;
use drone_cortexm::{reg::prelude::*, swo, thr::prelude::*};
use drone_stm32_map::periph::{
    dma::{periph_dma1, periph_dma1_ch5, periph_dma1_ch6},
    gpio::{
        periph_gpio_a2, periph_gpio_a3, periph_gpio_a_head, periph_gpio_b2,
        periph_gpio_b_head,
    },
    uart::periph_usart2,
};
use drone_stm32f4_hal::{
    dma::{config::*, DmaCfg},
    gpio::{GpioHead, GpioPinSpeed, prelude::*},
    rcc::{prelude::*, periph_flash, periph_pwr, periph_rcc, Flash, Pwr, Rcc, RccSetup},
    uart::{self, prelude::*},
};

/// The root task handler.
#[inline(never)]
pub fn handler(reg: Regs, thr_init: ThrsInit) {
    let thr = thr::init(thr_init);

    thr.hard_fault.add_once(|| panic!("Hard Fault"));

    println!("Hello, world!");

    // Enable interrupts.
    thr.rcc.enable_int();
    thr.usart2.enable_int();
    thr.dma1_ch5.enable_int();
    thr.dma1_ch6.enable_int();

    // Enable IO port clock.
    let port_a = GpioHead::with_enabled_clock(periph_gpio_a_head!(reg));

    // Configure UART GPIO pins.
    let pin_tx = port_a.pin(periph_gpio_a2!(reg))
        .into_alternate()
        .into_pushpull()
        .with_speed(GpioPinSpeed::HighSpeed);
    let pin_rx = port_a.pin(periph_gpio_a3!(reg))
        .into_alternate()
        .into_pushpull()
        .with_speed(GpioPinSpeed::HighSpeed);

    unsafe {
        port_a.disable_clock();
    }

    // Configure debug pins used for capturing logic analyzer shots.
    let gpio_b = GpioHead::with_enabled_clock(periph_gpio_b_head!(reg));

    let mut dbg1 = gpio_b.pin(periph_gpio_b2!(reg))
        .into_output()
        .into_pushpull()
        .with_speed(GpioPinSpeed::HighSpeed);

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
    let dma1 = DmaCfg::with_enabled_clock(periph_dma1!(reg));
    let rx_dma = dma1.ch(DmaChSetup::new(periph_dma1_ch5!(reg), thr.dma1_ch5));
    let tx_dma = dma1.ch(DmaChSetup::new(periph_dma1_ch6!(reg), thr.dma1_ch6));

    // Initialize uart.
    let uart_pins = uart::UartPins::default()
        .tx(pin_tx)
        .rx(pin_rx);
    let setup = uart::UartSetup::init(periph_usart2!(reg), thr.usart2, pclk1);
    let uart_drv = uart::UartDrv::init(setup);
    // let mut tx_drv = uart_drv.into_tx(tx_dma, &uart_pins);
    // let mut rx_drv = uart_drv.into_rx(rx_dma, &uart_pins);
    let (mut tx_drv, mut rx_drv) = uart_drv.into_trx(tx_dma, rx_dma, &uart_pins);

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
