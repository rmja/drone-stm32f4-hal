use crate::{
    diverged::{DmaChDiverged, UartDiverged},
};
use drone_cortexm::{fib, reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::{
    dma::ch::{traits::*, DmaChMap},
    uart::{traits::*, UartMap},
};
use futures::prelude::*;

pub struct TxGuard<'a, Uart: UartMap, UartInt: IntToken, DmaTx: DmaChMap, DmaTxInt: IntToken> {
    uart: &'a UartDiverged<Uart>,
    uart_int: &'a UartInt,
    dma_tx: &'a DmaChDiverged<DmaTx>,
    dma_tx_int: &'a DmaTxInt,
}

impl<'a, Uart: UartMap, UartInt: IntToken, DmaTx: DmaChMap, DmaTxInt: IntToken>
    TxGuard<'a, Uart, UartInt, DmaTx, DmaTxInt>
{
    pub(crate) fn new(
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