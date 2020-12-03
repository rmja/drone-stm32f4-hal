use crate::diverged::{DmaChDiverged, UartDiverged};
use drone_cortexm::{fib, reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::{
    dma::ch::{traits::*, DmaChMap, DmaChPeriph},
    uart::{traits::*, UartMap, UartPeriph},
};
use futures::prelude::*;

/// Uart setup.
pub struct UartSetup<Uart: UartMap, UartInt: IntToken, DmaTx: DmaChMap, DmaTxInt: IntToken, DmaRx: DmaChMap, DmaRxInt: IntToken> {
    /// Uart peripheral.
    pub uart: UartPeriph<Uart>,
    /// Uart global interrupt.
    pub uart_int: UartInt,
    /// Uart baud rate.
    pub uart_baud_rate: u32,
    /// Uart word length in bits.
    ///
    /// Valid values are 8 or 9.
    pub uart_word_length: u8,
    /// Uart stop bits.
    pub uart_stop_bits: UartStop,
    /// Uart parity.
    pub uart_parity: UartParity,
    /// Uart oversampling
    ///
    /// Valid values are 8 or 16.
    pub uart_oversampling: u8,
    
    /// DMA TX channel peripheral.
    pub dma_tx: DmaChPeriph<DmaTx>,
    /// DMA TX channel interrupt.
    pub dma_tx_int: DmaTxInt,
    /// DMA TX channel number.
    pub dma_tx_ch: u32,
    /// DMA TX channel priority level.
    pub dma_tx_pl: u32,

    /// DMA RX channel peripheral.
    pub dma_rx: DmaChPeriph<DmaRx>,
    /// DMA RX channel interrupt.
    pub dma_rx_int: DmaRxInt,
    /// DMA RX channel number.
    pub dma_rx_ch: u32,
    /// DMA RX channel priority level.
    pub dma_rx_pl: u32,
}

/// Uart stop bits.
#[derive(Clone, Copy, PartialEq)]
pub enum UartStop {
    Half,
    One,
    OneHalf,
    Two,
}

/// Uart parity.
#[derive(Clone, Copy, PartialEq)]
pub enum UartParity {
    None,
    Even,
    Odd,
}

/// Uart driver.
pub struct UartDrv<Uart: UartMap, UartInt: IntToken, DmaTx: DmaChMap, DmaTxInt: IntToken, DmaRx: DmaChMap, DmaRxInt: IntToken> {
    uart: UartDiverged<Uart>,
    uart_int: UartInt,
    dma_tx: DmaChDiverged<DmaTx>,
    dma_tx_int: DmaTxInt,
    dma_rx: DmaChDiverged<DmaRx>,
    dma_rx_int: DmaRxInt,
}

impl<Uart: UartMap, UartInt: IntToken, DmaTx: DmaChMap, DmaTxInt: IntToken, DmaRx: DmaChMap, DmaRxInt: IntToken>
    UartDrv<Uart, UartInt, DmaTx, DmaTxInt, DmaRx, DmaRxInt>
{
    /// Sets up a new [`UartDrv`] from `setup` values.
    #[must_use]
    pub fn init(setup: UartSetup<Uart, UartInt, DmaTx, DmaTxInt, DmaRx, DmaRxInt>) -> Self {
        let UartSetup {
            uart,
            uart_int,
            uart_baud_rate,
            uart_word_length,
            uart_stop_bits,
            uart_parity,
            uart_oversampling,
            dma_tx,
            dma_tx_int,
            dma_tx_ch,
            dma_tx_pl,
            dma_rx,
            dma_rx_int,
            dma_rx_ch,
            dma_rx_pl,
        } = setup;
        let mut drv = Self {
            uart: uart.into(),
            uart_int,
            dma_tx: dma_tx.into(),
            dma_tx_int,
            dma_rx: dma_rx.into(),
            dma_rx_int,
        };
        drv.init_uart(
            uart_baud_rate,
            uart_word_length,
            uart_stop_bits,
            uart_parity,
            uart_oversampling,
        );
        drv.init_dma_tx(dma_tx_ch, dma_tx_pl);
        drv.init_dma_rx(dma_rx_ch, dma_rx_pl);
        drv
    }

    pub fn tx(&self) -> TxGuard<Uart, UartInt, DmaTx, DmaTxInt> {
        TxGuard::new(&self.uart, &self.uart_int, &self.dma_tx, &self.dma_tx_int)
    }

    pub fn rx(&self, buf: Box<[u8]>) -> RxGuard<Uart, DmaRx, DmaRxInt> {
        RxGuard::new(&self.uart, &self.dma_rx, &self.dma_rx_int, buf)
    }

    fn init_uart(
        &mut self,
        baud_rate: u32,
        word_length: u8,
        stop_bits: UartStop,
        parity: UartParity,
        oversampling: u8,
    ) {
        // Enable uart clock.
        self.uart.rcc_busenr_uarten.set_bit();

        // Configure uart.
        self.uart.uart_cr1.store_reg(|r, v| {
            // Do not enable uart before it is fully configured.

            // Word length.
            if word_length == 9 {
                r.m().set(v);
            }

            // Parity.
            if parity != UartParity::None {
                // Enable parity.
                r.pce().set(v);
                if parity == UartParity::Odd {
                    // Parity selection: odd.
                    r.ps().set(v);
                }
            }

            // Oversampling.
            if oversampling == 8 {
                r.over8().set(v);
            }
        });
        self.uart.uart_cr2.store_reg(|r, v| {
            // Stop bits.
            r.stop().write(
                v,
                match stop_bits {
                    UartStop::One => 0,
                    UartStop::Half => 1,
                    UartStop::Two => 2,
                    UartStop::OneHalf => 3,
                },
            );
        });
        self.uart.uart_brr.store_reg(|r, v| {
            // Baud rate.
            // TODO: How to obtain correct clock instead of using hardcoded value?
            let (div_man, div_frac) = compute_brr(90_000_000, oversampling == 8, baud_rate);
            r.div_mantissa().write(v, div_man);
            r.div_fraction().write(v, div_frac);
        });

        self.uart.uart_cr1.modify_reg(|r, v| {
            // Enable parity error interrupt
            r.peie().set(v);
            // Enable ORE or RXNE interrupt
            r.rxneie().set(v);
            // Enable uart after being fully configured.
            r.ue().set(v);
        });

        // Attach uart error handler
        let sr = self.uart.uart_sr;
        self.uart_int.add_fn(move || {
            let val = sr.load_val();
            handle_uart_err::<Uart>(&val, sr);
            fib::Yielded::<(), !>(())
        });
    }

    fn init_dma_tx(&mut self, channel: u32, priority: u32) {
        let address = self.uart.uart_dr.as_mut_ptr(); // 8-bit data register
        self.dma_tx.dma_cpar.store_reg(|r, v| {
            r.pa().write(v, address as u32); // peripheral address
        });
        self.dma_tx.dma_ccr.store_reg(|r, v| {
            r.chsel().write(v, channel); // channel selection
            r.pl().write(v, priority); // priority level
            r.msize().write(v, 0b00); // byte (8-bit)
            r.psize().write(v, 0b00); // byte (8-bit)
            r.minc().set(v); // memory address pointer is incremented after each data transfer
            r.pinc().clear(v); // peripheral address pointer is fixed
            r.circ().clear(v); // normal mode.
            r.dir().write(v, 0b01); // memory-to-peripheral
            r.tcie().set(v); // transfer complete interrupt enable
            r.teie().set(v); // transfer error interrupt enable
        });

        // Attach dma error handler
        let dma_isr_dmeif = self.dma_tx.dma_isr_dmeif;
        let dma_isr_feif = self.dma_tx.dma_isr_feif;
        let dma_isr_teif = self.dma_tx.dma_isr_teif;
        self.dma_tx_int.add_fn(move || {
            // Load _entire_ interrupt status register.
            // The value is not masked to TEIF.
            let val = dma_isr_teif.load_val();
            handle_dma_err::<DmaTx>(&val, dma_isr_dmeif, dma_isr_feif, dma_isr_teif);
            fib::Yielded::<(), !>(())
        });
    }

    fn init_dma_rx(&mut self, channel: u32, priority: u32) {
        let address = self.uart.uart_dr.as_mut_ptr(); // 8-bit data register
        self.dma_rx.dma_cpar.store_reg(|r, v| {
            r.pa().write(v, address as u32); // peripheral address
        });
        self.dma_rx.dma_ccr.store_reg(|r, v| {
            r.chsel().write(v, channel); // channel selection
            r.pl().write(v, priority); // priority level
            r.msize().write(v, 0b00); // byte (8-bit)
            r.psize().write(v, 0b00); // byte (8-bit)
            r.minc().set(v); // memory address pointer is incremented after each data transfer
            r.pinc().clear(v); // peripheral address pointer is fixed
            r.circ().set(v); // circular mode.
            r.dir().write(v, 0b00); // peripheral-to-memory
            r.tcie().clear(v); // transfer complete interrupt disable
            r.teie().set(v); // transfer error interrupt enable
        });

        // Attach dma error handler
        let dma_isr_dmeif = self.dma_rx.dma_isr_dmeif;
        let dma_isr_feif = self.dma_rx.dma_isr_feif;
        let dma_isr_teif = self.dma_rx.dma_isr_teif;
        self.dma_tx_int.add_fn(move || {
            // Load _entire_ interrupt status register.
            // The value is not masked to TEIF.
            let val = dma_isr_teif.load_val();
            handle_dma_err::<DmaRx>(&val, dma_isr_dmeif, dma_isr_feif, dma_isr_teif);
            fib::Yielded::<(), !>(())
        });
    }
}

pub struct TxGuard<'a, Uart: UartMap, UartInt: IntToken, DmaTx: DmaChMap, DmaTxInt: IntToken> {
    uart: &'a UartDiverged<Uart>,
    uart_int: &'a UartInt,
    dma_tx: &'a DmaChDiverged<DmaTx>,
    dma_tx_int: &'a DmaTxInt,
}

impl<'a, Uart: UartMap, UartInt: IntToken, DmaTx: DmaChMap, DmaTxInt: IntToken>
    TxGuard<'a, Uart, UartInt, DmaTx, DmaTxInt>
{
    fn new(
        uart: &'a UartDiverged<Uart>,
        uart_int: &'a UartInt,
        dma_tx: &'a DmaChDiverged<DmaTx>,
        dma_tx_int: &'a DmaTxInt,
    ) -> Self {
        // Enable transmitter.
        uart.uart_cr1.modify_reg(|r, v| {
            r.te().set(v);
        });

        Self {
            uart,
            uart_int,
            dma_tx,
            dma_tx_int,
        }
    }

    /// Write a buffer using DMA to the uart peripheral.
    ///
    /// The write future completes when the DMA transfer has completed,
    /// at which time the peripheral is ready for another invokation of write().
    pub async fn write(&mut self, buf: &[u8]) {
        unsafe {
            self.write_unsafe(buf).await;
        }
    }

    unsafe fn write_unsafe(&mut self, buf: &[u8]) -> impl Future<Output = ()> {
        // PE (Parity error),
        // FE (Framing error),
        // NE (Noise error),
        // ORE (Overrun error), and
        // IDLE (Idle line detected) flags are cleared by the software sequence:
        // 1. a read operation to USART_SR register, followed by
        // 2. a read operation to USART_DR register.
        // See RM0090 page 972.
        self.uart.uart_sr.load_val();
        self.uart.uart_dr.load_val();

        // Setup DMA transfer parameters.
        self.setup_dma(buf);

        // Start listen for DMA transfer to complete.
        // The transfer completes just after the second last byte is being sent on the wire.
        let dma_isr_tcif = self.dma_tx.dma_isr_tcif;
        let dma_ifcr_ctcif = self.dma_tx.dma_ifcr_ctcif;
        let dma_tc = self.dma_tx_int.add_future(fib::new_fn(move || {
            if dma_isr_tcif.read_bit() {
                // Clear transfer completed interrupt flag.
                dma_ifcr_ctcif.set_bit();
                fib::Complete(())
            } else {
                fib::Yielded(())
            }
        }));

        // The uart transmission complete flag (TC) is cleared
        // by the sequence: Read status register (SR) and write data register (DR).
        // We read the status register here, and the dma writes the DR.
        // self.uart.uart_sr.load_val();
        self.uart.uart_sr.tc().clear_bit();

        // Clear any outstanding fifo error interrupt flag by settings its clear register.
        self.dma_tx.dma_ifcr_cfeif.set_bit();

        // Start transfer on DMA channel.
        self.uart.uart_cr3.modify_reg(|r, v| {
            r.dmat().set(v);
        });

        // Wait for DMA transfer to complete.
        dma_tc

        // The peripheral automatically disables the DMA stream on completion without error.
    }

    /// Wait for the uart peripheral to actually complete the transfer.
    pub async fn flush(&mut self) {
        // The transfor is completed when:
        // 1) transmit buffer to become empty (TXE) is asserted, and
        // 2) transmission complete (TC) is asserted.
        let uart_sr = self.uart.uart_sr;
        let uart_tc = self.uart_int.add_future(fib::new_fn(move || {
            let sr_val = uart_sr.load_val();
            if uart_sr.txe().read(&sr_val) && uart_sr.tc().read(&sr_val) {
                // The TXE flag is automatically cleared
                uart_sr.tc().clear_bit();
                fib::Complete(())
            } else {
                fib::Yielded(())
            }
        }));

        // Enable transmission complete interrupt.
        // This may fire immediately if the transmission is already completed.
        self.uart.uart_cr1.modify_reg(|r, v| {
            r.tcie().set(v);
        });

        // Wait for transfer to complete.
        uart_tc.await;

        // Disable transmission complete interrupt.
        self.uart.uart_cr1.modify_reg(|r, v| {
            r.tcie().clear(v);
        });
    }

    unsafe fn setup_dma(&mut self, buf_tx: &[u8]) {
        // Set buffer memory addres
        self.dma_tx.dma_cm0ar.store_reg(|r, v| {
            r.m0a().write(v, buf_tx.as_ptr() as u32);
        });

        // Set number of bytes to transfer
        self.dma_tx.dma_cndtr.store_reg(|r, v| {
            r.ndt().write(v, buf_tx.len() as u32);
        });

        // Clear transfer complete interrupt flag
        self.dma_tx.dma_ifcr_ctcif.set_bit();

        // Enable stream
        self.dma_tx.dma_ccr.modify_reg(|r, v| r.en().set(v));
    }
}

impl<Uart: UartMap, UartInt: IntToken, DmaTx: DmaChMap, DmaTxInt: IntToken> Drop
    for TxGuard<'_, Uart, UartInt, DmaTx, DmaTxInt>
{
    /// Stop the transmitter.
    ///
    /// It is preferred that flush() is called before drop so that this will not actually block until transmission completes.
    fn drop(&mut self) {
        // Wait for
        // 1) transmit buffer to become empty (TXE), and
        // 2) for transmission to complete (TC).
        let uart_sr = self.uart.uart_sr;
        loop {
            let sr_val = uart_sr.load_val();
            if uart_sr.txe().read(&sr_val) && uart_sr.tc().read(&sr_val) {
                break;
            }
        }

        // Disable transmitter.
        self.uart.uart_cr1.modify_reg(|r, v| {
            r.te().clear(v);
        });
    }
}


pub struct RxGuard<'a, Uart: UartMap, DmaRx: DmaChMap, DmaRxInt: IntToken> {
    uart: &'a UartDiverged<Uart>,
    dma_rx: &'a DmaChDiverged<DmaRx>,
    dma_rx_int: &'a DmaRxInt,
    ring_buf: Box<[u8]>,
    first: usize,
}

impl<'a, Uart: UartMap, DmaRx: DmaChMap, DmaRxInt: IntToken>
    RxGuard<'a, Uart, DmaRx, DmaRxInt>
{
    fn new(
        uart: &'a UartDiverged<Uart>,
        dma_rx: &'a DmaChDiverged<DmaRx>,
        dma_rx_int: &'a DmaRxInt,
        buf: Box<[u8]>,
    ) -> Self {
        // Enable receiver.
        uart.uart_cr1.modify_reg(|r, v| {
            r.re().set(v);
        });
        
        let mut rx = Self {
            uart,
            dma_rx,
            dma_rx_int,
            ring_buf: buf,
            first: 0,
        };

        unsafe {
            rx.setup_dma();
        }

        rx
    }

    /// Read into buffer using DMA to the uart peripheral.
    pub async fn read(&mut self, buf: &mut [u8]) -> Result<usize, i32> {
        // RX Buffer layout:
        //
        // Without wraparound:                             With wraparound:
        //
        //  + buf                      +--- NDTR ---+       + buf    +------------ NDTR ------------+
        //  |                          |            |       |        |                              |
        //  v                          v            v       v        v                              v
        // +-----------------------------------------+     +-----------------------------------------+
        // |oooooooooooXXXXXXXXXXXXXXXXoooooooooooooo|     |XXXXXXXXXooooooooooooooooXXXXXXXXXXXXXXXX|
        // +-----------------------------------------+     +-----------------------------------------+
        //  ^          ^               ^                    ^        ^               ^
        //  |          |               |                    |        |               |
        //  +- first --+               |                    +- end --+               |
        //  |                          |                    |                        |
        //  +- end --------------------+                    +- first ----------------+


        // NDTR is auto-reloaded to the ring buffer size when it reaches 0.
        // The transfer completed interrupt flag (TCIF) is asserted when this happens.
        // We use this to 

        let ndtr = self.dma_rx.dma_cndtr.ndt().read_bits() as usize;
        let end = buf.len() - ndtr;

        let read = if self.first == end {
            // There currently no bytes available in the buffer.

            // Return a buffer overflow error if TCIF is asserted
            // as the DMA controller in that case has wrapped.
            if self.dma_rx.dma_isr_tcif.read_bit() {
                // Clear transfer completed interrupt flag.
                self.dma_rx.dma_ifcr_ctcif.set_bit();

                return Err(123);
            }

            // Listen for 
            // self.uart_cr1.modify_reg(|r, v| {
            //     r.tcie().set(v);
            // });

            0
        }
        else {
            // There are bytes readily available in the buffer.

            if self.first < end {
                // The available portion _does not_ wrap.

                // Return a buffer overflow error if TCIF is asserted
                // as the DMA controller in that case has wrapped.
                if self.dma_rx.dma_isr_tcif.read_bit() {
                    // Clear transfer completed interrupt flag.
                    self.dma_rx.dma_ifcr_ctcif.set_bit();

                    return Err(123);
                }

                let src = self.first..end;
                let dst = 0..buf.len();
                let cnt = core::cmp::min(src.len(), dst.len());
                buf[dst].copy_from_slice(&self.ring_buf[src]);
                self.first += cnt;

                cnt
            }
            else {
                // The available portion _does_ wrap.

                // Clear transfer completed interrupt flag.
                self.dma_rx.dma_ifcr_ctcif.set_bit();

                // Copy the tail.
                let src = self.first..self.ring_buf.len();
                let dst = 0..buf.len();
                let cnt_tail = core::cmp::min(src.len(), dst.len());
                buf[dst].copy_from_slice(&self.ring_buf[src]);

                // Copy the head.
                let src = 0..end;
                let dst = cnt_tail..buf.len();
                let cnt_head = core::cmp::min(src.len(), dst.len());
                buf[dst].copy_from_slice(&self.ring_buf[src]);

                cnt_tail + cnt_head
            }
        };

        Ok(read)
    }

    unsafe fn setup_dma(&mut self) {
        // Set buffer memory addres
        self.dma_rx.dma_cm0ar.store_reg(|r, v| {
            r.m0a().write(v, self.ring_buf.as_ptr() as u32);
        });

        // Set number of bytes to transfer
        self.dma_rx.dma_cndtr.store_reg(|r, v| {
            r.ndt().write(v, self.ring_buf.len() as u32);
        });

        // Enable stream
        self.dma_rx.dma_ccr.modify_reg(|r, v| r.en().set(v));
    }
}

impl<Uart: UartMap, DmaRx: DmaChMap, DmaRxInt: IntToken> Drop
    for RxGuard<'_, Uart, DmaRx, DmaRxInt>
{
    fn drop(&mut self) {
        // Disable receiver.
        self.uart.uart_cr1.modify_reg(|r, v| {
            r.re().clear(v);
        });
    }
}


fn compute_brr(clock: u32, over8: bool, baud_rate: u32) -> (u32, u32) {
    // see PM0090 §30.3.4 Fractional baud rate generation page 978
    let over8 = over8 as u32;
    // (25*clock) / 2*(2-over8)*baud_rate) === (100*clock) / (8*(2-over8)*baud_rate).
    // But we take the 25 version to ensure that 25 * clock can fit in a u32.
    let div100 = (25 * clock) / (2 * (2 - over8) * baud_rate);
    let div_man = div100 / 100;
    let div_frac = if over8 == 1 {
        // The frac field has 3 bits, 0..15
        ((div100 - div_man * 100) * 16 + 50) / 100
    } else {
        // The frac field has 4 bits
        ((div100 - div_man * 100) * 32 + 50) / 100
    };

    (div_man, div_frac)
}

fn handle_uart_err<Uart: UartMap>(val: &Uart::UartSrVal, sr: Uart::CUartSr) {
    if sr.rxne().read(&val) {
        panic!("Read data register not empty");
    }
    if sr.ore().read(&val) {
        panic!("Overrun error");
    }
    if sr.nf().read(&val) {
        panic!("Noice");
    }
    if sr.fe().read(&val) {
        panic!("Framing error");
    }
    if sr.pe().read(&val) {
        panic!("Parity error");
    }
}

fn handle_dma_err<T: DmaChMap>(
    val: &T::DmaIsrVal,
    dma_isr_dmeif: T::CDmaIsrDmeif,
    dma_isr_feif: T::CDmaIsrFeif,
    dma_isr_teif: T::CDmaIsrTeif,
) {
    if dma_isr_teif.read(&val) {
        panic!("Transfer error");
    }
    if dma_isr_dmeif.read(&val) {
        panic!("Direct mode error");
    }
    if dma_isr_feif.read(&val) {
        panic!("FIFO error");
    }
}
