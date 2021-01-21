use crate::diverged::{DmaChDiverged, SpiDiverged};
use drone_cortexm::{fib, reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::{
    dma::ch::DmaChMap,
    spi::{traits::*, SpiMap},
};
use drone_stm32f4_dma_drv::{DmaChCfg, DmaStChToken};

pub struct SpiMasterDrv<
    'drv,
    Spi: SpiMap,
    DmaRx: DmaChMap,
    DmaRxInt: IntToken,
    DmaTx: DmaChMap,
    DmaTxInt: IntToken,
> {
    pub(crate) spi: &'drv SpiDiverged<Spi>,
    pub(crate) dma_rx: DmaChDiverged<DmaRx>,
    pub(crate) dma_rx_int: DmaRxInt,
    pub(crate) dma_tx: DmaChDiverged<DmaTx>,
    pub(crate) dma_tx_int: DmaTxInt,
}

impl<
        'drv,
        Spi: SpiMap,
        DmaRx: DmaChMap,
        DmaRxInt: IntToken,
        DmaTx: DmaChMap,
        DmaTxInt: IntToken,
    > SpiMasterDrv<'drv, Spi, DmaRx, DmaRxInt, DmaTx, DmaTxInt>
{
    pub(crate) fn init<DmaRxStCh: DmaStChToken, DmaTxStCh: DmaStChToken>(
        spi: &'drv SpiDiverged<Spi>,
        miso_cfg: DmaChCfg<DmaRx, DmaRxStCh, DmaRxInt>,
        mosi_cfg: DmaChCfg<DmaTx, DmaTxStCh, DmaTxInt>,
    ) -> Self {
        let DmaChCfg {
            dma_ch: dma_rx,
            dma_int: dma_rx_int,
            dma_pl: dma_rx_pl,
            ..
        } = miso_cfg;
        let DmaChCfg {
            dma_ch: dma_tx,
            dma_int: dma_tx_int,
            dma_pl: dma_tx_pl,
            ..
        } = mosi_cfg;
        let master = Self {
            spi,
            dma_rx: dma_rx.into(),
            dma_rx_int,
            dma_tx: dma_tx.into(),
            dma_tx_int,
        };

        spi.spi_cr1.modify_reg(|r, v| {
            // Master configuration.
            r.mstr().set(v);

            // Use software slave management, i.e. the app controls slave selection.
            // The hardware NSS pin is free for other use.
            r.ssm().set(v);

            // Internal slave select (required for master operation when software slave management (SSM) is being used).
            r.ssi().set(v);

            // Enable spi after being fully configured.
            r.spe().set(v);
        });

        master
            .dma_rx
            .init_dma_rx(spi.spi_dr.as_mut_ptr() as u32, DmaRxStCh::num(), dma_rx_pl);
        master.dma_rx.panic_on_err(master.dma_rx_int);

        master
            .dma_tx
            .init_dma_tx(spi.spi_dr.as_mut_ptr() as u32, DmaTxStCh::num(), dma_tx_pl);
        master.dma_tx.panic_on_err(master.dma_tx_int);

        master
    }

    /// Send to the currently selected slave.
    pub async fn write(&mut self, buf: &[u8]) {
        if buf.is_empty() {
            return;
        }

        self.wait_for_idle();

        unsafe {
            // Setup DMA transfer parameters.
            self.dma_rx.setup_dummy_stream(buf.len());
            self.dma_tx.setup_stream(buf);

            self.xfer_impl().await;
        }
    }

    /// Read from the currently selected slave.
    pub async fn read(&mut self, buf: &mut [u8]) {
        if buf.is_empty() {
            return;
        }

        self.wait_for_idle();

        unsafe {
            self.dma_rx.setup_stream(buf);
            self.dma_tx.setup_dummy_stream(buf.len());

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
            self.dma_rx.setup_stream(rx_buf);
            self.dma_tx.setup_stream(tx_buf);

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

impl<Spi: SpiMap, DmaRx: DmaChMap, DmaRxInt: IntToken, DmaTx: DmaChMap, DmaTxInt: IntToken> Drop
    for SpiMasterDrv<'_, Spi, DmaRx, DmaRxInt, DmaTx, DmaTxInt>
{
    fn drop(&mut self) {
        self.wait_for_idle();

        self.spi.spi_cr1.modify_reg(|r, v| {
            // Disable spi.
            r.spe().clear(v);
        });
    }
}
