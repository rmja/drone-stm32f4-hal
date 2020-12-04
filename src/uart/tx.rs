use crate::diverged::{DmaChDiverged, UartDiverged};
use drone_cortexm::{fib, reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::{
    dma::ch::{traits::*, DmaChMap},
    uart::{traits::*, UartMap},
};

pub struct UartTxDrv<'drv, Uart: UartMap, UartInt: IntToken, DmaTx: DmaChMap, DmaTxInt: IntToken> {
    pub(crate) uart: &'drv UartDiverged<Uart>,
    pub(crate) uart_int: &'drv UartInt,
    pub(crate) dma: DmaChDiverged<DmaTx>,
    pub(crate) dma_int: DmaTxInt,
}

pub struct TxGuard<'sess, Uart: UartMap, UartInt: IntToken, DmaTx: DmaChMap, DmaTxInt: IntToken> {
    drv: &'sess UartTxDrv<'sess, Uart, UartInt, DmaTx, DmaTxInt>,
    busy: bool,
}

impl<'drv, Uart: UartMap, UartInt: IntToken, DmaTx: DmaChMap, DmaTxInt: IntToken>
    UartTxDrv<'drv, Uart, UartInt, DmaTx, DmaTxInt>
{
    pub(crate) fn init_dma_tx(&mut self, channel: u32, priority: u32) {
        let address = self.uart.uart_dr.as_mut_ptr(); // 8-bit data register
        self.dma.dma_cpar.store_reg(|r, v| {
            r.pa().write(v, address as u32); // peripheral address
        });
        self.dma.dma_ccr.store_reg(|r, v| {
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
        self.dma.dma_cfcr.store_reg(|r, v| {
            r.dmdis().clear(v); // use direct mode instead of fifo
        });

        // Attach dma error handler
        let dma_isr_dmeif = self.dma.dma_isr_dmeif;
        let dma_isr_feif = self.dma.dma_isr_feif;
        let dma_isr_teif = self.dma.dma_isr_teif;
        self.dma_int.add_fn(move || {
            // Load _entire_ interrupt status register.
            // The value is not masked to TEIF.
            let val = dma_isr_teif.load_val();
            crate::drv::handle_dma_err::<DmaTx>(&val, dma_isr_dmeif, dma_isr_feif, dma_isr_teif);
            fib::Yielded::<(), !>(())
        });
    }

    /// Enable tx operation for the uart peripheral and return a guard that disables the transmitter when dropped.
    pub fn sess<'sess>(&'sess mut self) -> TxGuard<'sess, Uart, UartInt, DmaTx, DmaTxInt> {
        // Enable transmitter.
        self.uart.uart_cr1.modify_reg(|r, v| {
            r.te().set(v);
        });

        TxGuard {
            drv: self,
            busy: false,
        }
    }
}

impl<'sess, Uart: UartMap, UartInt: IntToken, DmaTx: DmaChMap, DmaTxInt: IntToken>
    TxGuard<'sess, Uart, UartInt, DmaTx, DmaTxInt>
{
    /// Write a buffer using DMA to the uart peripheral.
    ///
    /// The write future completes when the DMA transfer has completed,
    /// at which time the peripheral is ready for another invokation of write().
    pub async fn write(&mut self, buf: &[u8]) {
        if buf.len() == 0 {
            return;
        }

        unsafe {
            self.write_unsafe(buf).await;
        }
    }

    async unsafe fn write_unsafe(&mut self, buf: &[u8]) {
        let drv = self.drv;
        // PE (Parity error),
        // FE (Framing error),
        // NE (Noise error),
        // ORE (Overrun error), and
        // IDLE (Idle line detected) flags are cleared by the software sequence:
        // 1. a read operation to USART_SR register, followed by
        // 2. a read operation to USART_DR register.
        // See RM0090 page 972.
        drv.uart.uart_sr.load_val();
        drv.uart.uart_dr.load_val();

        // Setup DMA transfer parameters.
        self.setup_dma(buf);

        // Start listen for DMA transfer to complete.
        // The transfer completes just after the second last byte is being sent on the wire.
        let dma_isr_tcif = drv.dma.dma_isr_tcif;
        let dma_ifcr_ctcif = drv.dma.dma_ifcr_ctcif;
        let dma_tc = drv.dma_int.add_future(fib::new_fn(move || {
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
        drv.uart.uart_sr.tc().clear_bit();

        // Clear any outstanding fifo error interrupt flag by settings its clear register.
        drv.dma.dma_ifcr_cfeif.set_bit();

        // Start transfer on DMA channel.
        drv.uart.uart_cr3.modify_reg(|r, v| {
            r.dmat().set(v);
        });

        self.busy = true;

        // Wait for DMA transfer to complete.
        dma_tc.await;

        // The peripheral automatically disables the DMA stream on completion without error,
        // but it does not clear the DMAT flag in CR3.

        // Stop transfer on DMA channel.
        drv.uart.uart_cr3.modify_reg(|r, v| {
            r.dmat().clear(v);
        });
    }

    /// Wait for the uart peripheral to actually complete the transfer.
    pub async fn flush(&mut self) {
        if !self.busy {
            // write() has not been called - there is nothing to wait for.
            return;
        }

        let drv = self.drv;
        // The transfor is completed when:
        // 1) transmit buffer empty (TXE) is asserted, and
        // 2) transmission complete (TC) is asserted.
        let uart_sr = drv.uart.uart_sr;
        let uart_tc = drv.uart_int.add_future(fib::new_fn(move || {
            let sr_val = uart_sr.load_val();
            if uart_sr.txe().read(&sr_val) && uart_sr.tc().read(&sr_val) {
                // The TXE flag is automatically cleared.
                uart_sr.tc().clear_bit();
                fib::Complete(())
            } else {
                fib::Yielded(())
            }
        }));

        // Enable transmission complete interrupt.
        // This may fire immediately if the transmission is already completed.
        drv.uart.uart_cr1.modify_reg(|r, v| {
            r.tcie().set(v);
        });

        // Wait for transfer to complete.
        uart_tc.await;

        // Disable transmission complete interrupt.
        drv.uart.uart_cr1.modify_reg(|r, v| {
            r.tcie().clear(v);
        });

        // Wait for another call to write() before we need to wait in flush().
        self.busy = false;
    }

    unsafe fn setup_dma(&mut self, buf_tx: &[u8]) {
        let drv = self.drv;

        // Set buffer memory addres.
        drv.dma.dma_cm0ar.store_reg(|r, v| {
            r.m0a().write(v, buf_tx.as_ptr() as u32);
        });

        // Set number of bytes to transfer.
        drv.dma.dma_cndtr.store_reg(|r, v| {
            r.ndt().write(v, buf_tx.len() as u32);
        });

        // Clear transfer complete interrupt flag.
        drv.dma.dma_ifcr_ctcif.set_bit();

        // Enable stream.
        drv.dma.dma_ccr.modify_reg(|r, v| r.en().set(v));
    }
}

impl<Uart: UartMap, UartInt: IntToken, DmaTx: DmaChMap, DmaTxInt: IntToken> Drop
    for TxGuard<'_, Uart, UartInt, DmaTx, DmaTxInt>
{
    /// Stop the transmitter.
    ///
    /// It is preferred that flush() is called before drop so that this will not actually block until transmission completes.
    fn drop(&mut self) {
        let drv = self.drv;

        if self.busy {
            // Wait for
            // 1) transmit buffer to become empty (TXE), and
            // 2) for transmission to complete (TC).
            let uart_sr = drv.uart.uart_sr;
            loop {
                let sr_val = uart_sr.load_val();
                if uart_sr.txe().read(&sr_val) && uart_sr.tc().read(&sr_val) {
                    break;
                }
            }

            self.busy = false;
        }

        // Disable transmitter.
        drv.uart.uart_cr1.modify_reg(|r, v| {
            r.te().clear(v);
        });
    }
}
