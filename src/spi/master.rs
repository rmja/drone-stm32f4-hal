use crate::diverged::{DmaChDiverged, SpiDiverged};
use drone_cortexm::{fib, reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::{
    dma::ch::{traits::*, DmaChMap},
    spi::SpiMap,
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
    pub(crate) fn init_dma_rx(&mut self, channel: u32, priority: u32) {
        let address = self.spi.spi_dr.as_mut_ptr(); // 8-bit data register
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

    pub(crate) fn init_dma_tx(&mut self, channel: u32, priority: u32) {
        let address = self.spi.spi_dr.as_mut_ptr(); // 8-bit data register
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
            crate::drv::handle_dma_err::<DmaTx>(&val, dma_isr_dmeif, dma_isr_feif, dma_isr_teif);
            fib::Yielded::<(), !>(())
        });
    }

    /// Send a buffer to the currently selected slave.
    pub async fn send(&mut self, tx_buf: &[u8]) -> usize {
        tx_buf.len()
    }

    /// Send to and receive from the currently selected slave.
    pub async fn xfer(&mut self, tx_buf: &[u8], rx_buf: &mut &[u8]) -> usize {
        assert!(rx_buf.len() >= tx_buf.len());

        tx_buf.len()
    }

    /// Read the current value of the miso pin.
    pub fn miso(&self) -> bool {
        true
    }
}
