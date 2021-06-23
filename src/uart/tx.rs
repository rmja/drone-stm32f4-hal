use crate::diverged::{DmaChDiverged, UartDiverged};
use alloc::sync::Arc;
use drone_cortexm::{fib, reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::{
    dma::ch::DmaChMap,
    uart::{traits::*, UartMap},
};
use drone_stm32f4_dma_drv::{DmaChCfg, DmaStChToken};

pub struct UartTxDrv<Uart: UartMap, UartInt: IntToken, DmaTx: DmaChMap, DmaTxInt: IntToken> {
    pub(crate) uart: Arc<UartDiverged<Uart>>,
    pub(crate) uart_int: UartInt,
    pub(crate) dma: DmaChDiverged<DmaTx>,
    pub(crate) dma_int: DmaTxInt,
}

pub struct TxGuard<'sess, Uart: UartMap, UartInt: IntToken, DmaTx: DmaChMap, DmaTxInt: IntToken> {
    drv: &'sess UartTxDrv<Uart, UartInt, DmaTx, DmaTxInt>,
    busy: bool,
}

impl<Uart: UartMap, UartInt: IntToken, DmaTx: DmaChMap, DmaTxInt: IntToken>
    UartTxDrv<Uart, UartInt, DmaTx, DmaTxInt>
{
    pub(crate) fn init<DmaTxStCh: DmaStChToken>(
        uart: Arc<UartDiverged<Uart>>,
        uart_int: UartInt,
        tx_cfg: DmaChCfg<DmaTx, DmaTxStCh, DmaTxInt>,
    ) -> Self {
        let DmaChCfg {
            dma_ch,
            dma_int,
            dma_pl,
            ..
        } = tx_cfg;
        let tx = Self {
            uart,
            uart_int,
            dma: dma_ch.into(),
            dma_int,
        };
        tx.dma
            .init_dma_tx(tx.uart.uart_dr.as_mut_ptr() as u32, DmaTxStCh::NUM, dma_pl);
        tx.dma.panic_on_err(dma_int);
        tx
    }

    /// Enable tx operation for the uart peripheral and return a guard that disables the transmitter when dropped.
    pub fn start(&mut self) -> TxGuard<Uart, UartInt, DmaTx, DmaTxInt> {
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
        if buf.is_empty() {
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
        drv.dma.setup_stream(buf);

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
        drv.uart.uart_sr.load_val();

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
