use crate::diverged::{DmaChDiverged, SpiDiverged};
use drone_cortexm::{fib, reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::{
    dma::ch::{traits::*, DmaChMap},
    spi::{traits::*, SpiMap},
};

pub struct SpiMasterDrv<
    'drv,
    Spi: SpiMap,
    SpiInt: IntToken,
    DmaRx: DmaChMap,
    DmaRxInt: IntToken,
    DmaTx: DmaChMap,
    DmaTxInt: IntToken,
> {
    pub(crate) spi: &'drv SpiDiverged<Spi>,
    pub(crate) spi_int: &'drv SpiInt,
    pub(crate) dma_rx: DmaChDiverged<DmaRx>,
    pub(crate) dma_rx_int: DmaRxInt,
    pub(crate) dma_tx: DmaChDiverged<DmaTx>,
    pub(crate) dma_tx_int: DmaTxInt,
}

impl<
        'drv,
        Spi: SpiMap,
        SpiInt: IntToken,
        DmaRx: DmaChMap,
        DmaRxInt: IntToken,
        DmaTx: DmaChMap,
        DmaTxInt: IntToken,
    > SpiMasterDrv<'drv, Spi, SpiInt, DmaRx, DmaRxInt, DmaTx, DmaTxInt>
{
    pub(crate) fn init_dma_rx(&mut self, chsel: u32, priority: u32) {
        let address = self.spi.spi_dr.as_mut_ptr(); // 8-bit data register
        self.dma_rx.dma_cpar.store_reg(|r, v| {
            r.pa().write(v, address as u32); // peripheral address
        });
        self.dma_rx.dma_ccr.store_reg(|r, v| {
            r.chsel().write(v, chsel); // channel selection
            r.pl().write(v, priority); // priority level
            r.msize().write(v, 0b00); // byte (8-bit)
            r.psize().write(v, 0b00); // byte (8-bit)
            r.minc().set(v); // memory address pointer is incremented after each data transfer
            r.pinc().clear(v); // peripheral address pointer is fixed
            r.dir().write(v, 0b00); // peripheral-to-memory
            r.tcie().clear(v); // transfer complete interrupt disable
            r.teie().set(v); // transfer error interrupt enable
        });

        // Attach dma error handler
        let dma_isr_dmeif = self.dma_rx.dma_isr_dmeif;
        let dma_isr_feif = self.dma_rx.dma_isr_feif;
        let dma_isr_teif = self.dma_rx.dma_isr_teif;
        self.dma_rx_int.add_fn(move || {
            // Load _entire_ interrupt status register.
            // The value is not masked to TEIF.
            let val = dma_isr_teif.load_val();
            crate::drv::handle_dma_err::<DmaRx>(&val, dma_isr_dmeif, dma_isr_feif, dma_isr_teif);
            fib::Yielded::<(), !>(())
        });
    }

    pub(crate) fn init_dma_tx(&mut self, chsel: u32, priority: u32) {
        let address = self.spi.spi_dr.as_mut_ptr(); // 8-bit data register
        self.dma_tx.dma_cpar.store_reg(|r, v| {
            r.pa().write(v, address as u32); // peripheral address
        });
        self.dma_tx.dma_ccr.store_reg(|r, v| {
            r.chsel().write(v, chsel); // channel selection
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
            crate::drv::handle_dma_err::<DmaTx>(&val, dma_isr_dmeif, dma_isr_feif, dma_isr_teif);
            fib::Yielded::<(), !>(())
        });
    }

    /// Send a buffer to the currently selected slave.
    pub async fn write(&mut self, buf: &[u8]) {
        if buf.is_empty() {
            return;
        }

        unsafe {
            self.writeonly_unsafe(buf).await;
        }
    }

    pub async fn read(&mut self, buf: &mut [u8]) {
        if buf.is_empty() {
            return;
        }

        unsafe {
            self.readonly_unsafe(buf).await;
        }
    }

    /// Send to and receive from the currently selected slave.
    pub async fn xfer(&mut self, tx_buf: &[u8], rx_buf: &mut [u8]) {
        assert!(rx_buf.len() == tx_buf.len());

        if tx_buf.is_empty() {
            return;
        }

        unsafe {
            self.xfer_unsafe(tx_buf, rx_buf).await;
        }
    }

    async unsafe fn writeonly_unsafe(&mut self, buf: &[u8]) {
        // Setup DMA transfer parameters.
        setup_dma(&mut self.dma_tx, buf);

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

        // Clear any outstanding fifo error interrupt flag by settings its clear register.
        self.dma_tx.dma_ifcr_cfeif.set_bit();

        // Start transfer on DMA channel.
        self.spi.spi_cr2.modify_reg(|r, v| {
            r.txdmaen().set(v);
        });

        // Wait for DMA transfer to complete.
        dma_tc.await;

        // The peripheral automatically disables the DMA stream on completion without error,
        // but it does not clear the DMAT flag in CR3.

        // Stop transfer on DMA channel.
        self.spi.spi_cr2.modify_reg(|r, v| {
            r.txdmaen().clear(v);
        });
    }

    async unsafe fn readonly_unsafe(&mut self, buf: &mut [u8]) {
        // Setup DMA transfer parameters.
        setup_dma(&mut self.dma_rx, buf);

        // Start listen for DMA transfer to complete.
        // The transfer completes just after the second last byte is being sent on the wire.
        let dma_isr_tcif = self.dma_rx.dma_isr_tcif;
        let dma_ifcr_ctcif = self.dma_rx.dma_ifcr_ctcif;
        let dma_tc = self.dma_rx_int.add_future(fib::new_fn(move || {
            if dma_isr_tcif.read_bit() {
                // Clear transfer completed interrupt flag.
                dma_ifcr_ctcif.set_bit();
                fib::Complete(())
            } else {
                fib::Yielded(())
            }
        }));

        // Clear any outstanding fifo error interrupt flag by settings its clear register.
        self.dma_rx.dma_ifcr_cfeif.set_bit();

        // Start transfer on DMA channel.
        self.spi.spi_cr2.modify_reg(|r, v| {
            r.rxdmaen().set(v);
        });

        // Wait for DMA transfer to complete.
        dma_tc.await;

        // The peripheral automatically disables the DMA stream on completion without error,
        // but it does not clear the DMAT flag in CR3.

        // Stop transfer on DMA channel.
        self.spi.spi_cr2.modify_reg(|r, v| {
            r.rxdmaen().clear(v);
        });
    }

    async unsafe fn xfer_unsafe(&mut self, tx_buf: &[u8], rx_buf: &mut [u8]) {
        // Setup DMA transfer parameters.
        setup_dma(&mut self.dma_rx, rx_buf);
        setup_dma(&mut self.dma_tx, tx_buf);

        // Start listen for RX DMA transfer to complete.
        // The transfer completes just after the second last byte is being sent on the wire.
        let dma_isr_tcif = self.dma_rx.dma_isr_tcif;
        let dma_ifcr_ctcif = self.dma_rx.dma_ifcr_ctcif;
        let dma_rx_tc = self.dma_rx_int.add_future(fib::new_fn(move || {
            if dma_isr_tcif.read_bit() {
                // Clear transfer completed interrupt flag.
                dma_ifcr_ctcif.set_bit();
                fib::Complete(())
            } else {
                fib::Yielded(())
            }
        }));

        // Clear any outstanding fifo error interrupt flag by settings its clear register.
        self.dma_rx.dma_ifcr_cfeif.set_bit();
        self.dma_tx.dma_ifcr_cfeif.set_bit();

        // Start transfer on DMA channel.
        self.spi.spi_cr2.modify_reg(|r, v| {
            r.rxdmaen().set(v);
            r.txdmaen().set(v);
        });

        // Wait for DMA transfer to complete.
        dma_rx_tc.await;

        // The peripheral automatically disables the DMA stream on completion without error,
        // but it does not clear the DMAT flag in CR3.

        // Stop transfer on DMA channel.
        self.spi.spi_cr2.modify_reg(|r, v| {
            r.rxdmaen().clear(v);
            r.txdmaen().clear(v);
        });
    }

    /// Read the current value of the miso pin.
    pub fn miso(&self) -> bool {
        todo!();
    }
}

unsafe fn setup_dma<Dma: DmaChMap>(dma: &mut DmaChDiverged<Dma>, buf: &[u8]) {
    // Set buffer memory addres.
    dma.dma_cm0ar.store_reg(|r, v| {
        r.m0a().write(v, buf.as_ptr() as u32);
    });

    // Set number of bytes to transfer.
    dma.dma_cndtr.store_reg(|r, v| {
        r.ndt().write(v, buf.len() as u32);
    });

    // Clear transfer complete interrupt flag.
    dma.dma_ifcr_ctcif.set_bit();

    // Enable stream.
    dma.dma_ccr.modify_reg(|r, v| r.en().set(v));
}
