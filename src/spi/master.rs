use crate::diverged::{DmaChDiverged, SpiDiverged};
use drone_cortexm::{fib, reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::{
    dma::ch::{traits::*, DmaChMap},
    spi::{traits::*, SpiMap, SpiCr1},
};

pub struct SpiMasterDrv<
    'drv,
    Spi: SpiMap + SpiCr1,
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
        Spi: SpiMap + SpiCr1,
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
            // r.minc().set(v); // memory address pointer is incremented after each data transfer
            r.pinc().clear(v); // peripheral address pointer is fixed
            r.dir().write(v, 0b00); // peripheral-to-memory
            r.tcie().set(v); // transfer complete interrupt enable
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
            // r.minc().set(v); // memory address pointer is incremented after each data transfer
            r.pinc().clear(v); // peripheral address pointer is fixed
            r.circ().clear(v); // normal mode.
            r.dir().write(v, 0b01); // memory-to-peripheral
            r.tcie().clear(v); // transfer complete interrupt disable
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

        self.wait_for_idle();

        unsafe {
            // Setup DMA transfer parameters.
            let void_buf = [0u8];
            setup_dma_void(&mut self.dma_rx, &void_buf, buf.len());
            setup_dma(&mut self.dma_tx, buf);

            self.xfer_impl().await;
        }
    }

    pub async fn read(&mut self, buf: &mut [u8]) {
        if buf.is_empty() {
            return;
        }

        self.wait_for_idle();

        unsafe {
            let void_buf = [0u8];
            setup_dma(&mut self.dma_rx, buf);
            setup_dma_void(&mut self.dma_tx, &void_buf, buf.len());
            self.xfer_impl().await;
        }
    }

    /// Send to and receive from the currently selected slave.
    pub async fn xfer(&mut self, tx_buf: &[u8], rx_buf: &mut [u8]) {
        assert_eq!(tx_buf.len(), rx_buf.len());

        if tx_buf.is_empty() {
            return;
        }

        self.wait_for_idle();

        unsafe {
            setup_dma(&mut self.dma_rx, rx_buf);
            setup_dma(&mut self.dma_tx, tx_buf);

            self.xfer_impl().await;
        }
    }

    fn wait_for_idle(&self) {
        loop {
            let spi_sr = self.spi.spi_sr;
            let sr_val = spi_sr.load_val();
            if spi_sr.txe().read(&sr_val) && !spi_sr.bsy().read(&sr_val) {
                break;
            }
        }
    }

    async unsafe fn xfer_impl(&mut self) {
        // Start listen for rx dma transfer to complete.
        // Rx completion is guaranteed to always happen after tx has completed.
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
        // but it does not clear the RXDMAEN/TXDMAEN flag in CR2.

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

impl<
    Spi: SpiMap + SpiCr1,
    SpiInt: IntToken,
    DmaRx: DmaChMap,
    DmaRxInt: IntToken,
    DmaTx: DmaChMap,
    DmaTxInt: IntToken,
> Drop for SpiMasterDrv<'_, Spi, SpiInt, DmaRx, DmaRxInt, DmaTx, DmaTxInt> {
    fn drop(&mut self) {
        self.wait_for_idle();

        self.spi.spi_cr1.modify_reg(|r, v| {
            // Disable spi.
            r.spe().clear(v);
        });
    }
}

unsafe fn setup_dma<Dma: DmaChMap>(dma: &mut DmaChDiverged<Dma>, buf: &[u8]) {
    dma.dma_ccr.modify_reg(|r, v| {
        r.minc().set(v); // memory address pointer is incremented after each data transfer
    });

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

unsafe fn setup_dma_void<Dma: DmaChMap>(dma: &mut DmaChDiverged<Dma>, void_buf: &[u8; 1], len: usize) {
    dma.dma_ccr.modify_reg(|r, v| {
        r.minc().clear(v); // memory address pointer is fixed
    });

    // Set buffer memory addres.
    dma.dma_cm0ar.store_reg(|r, v| {
        r.m0a().write(v, void_buf.as_ptr() as u32);
    });

    // Set number of bytes to transfer.
    dma.dma_cndtr.store_reg(|r, v| {
        r.ndt().write(v, len as u32);
    });

    // Clear transfer complete interrupt flag.
    dma.dma_ifcr_ctcif.set_bit();

    // Enable stream.
    dma.dma_ccr.modify_reg(|r, v| r.en().set(v));
}
