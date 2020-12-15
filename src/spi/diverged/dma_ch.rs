use drone_cortexm::{fib, reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::dma::ch::{DmaChMap, DmaChPeriph, SDmaCm0Ar, SDmaCcr, SDmaCpar, CDmaCndtr};

#[allow(dead_code)]
pub(crate) struct DmaChDiverged<DmaCh: DmaChMap> {
    pub(crate) dma_ccr: DmaCh::SDmaCcr,
    pub(crate) dma_cfcr: DmaCh::SDmaCfcr,
    pub(crate) dma_cm0ar: DmaCh::SDmaCm0Ar,
    pub(crate) dma_cm1ar: DmaCh::SDmaCm1Ar,
    pub(crate) dma_cndtr: DmaCh::CDmaCndtr,
    pub(crate) dma_cpar: DmaCh::SDmaCpar,
    pub(crate) dma_ifcr_cdmeif: DmaCh::SDmaIfcrCdmeif,
    pub(crate) dma_ifcr_cfeif: DmaCh::SDmaIfcrCfeif,
    pub(crate) dma_ifcr_chtif: DmaCh::SDmaIfcrChtif,
    pub(crate) dma_ifcr_ctcif: DmaCh::CDmaIfcrCtcif,
    pub(crate) dma_ifcr_cteif: DmaCh::SDmaIfcrCteif,
    pub(crate) dma_isr_dmeif: DmaCh::CDmaIsrDmeif,
    pub(crate) dma_isr_feif: DmaCh::CDmaIsrFeif,
    pub(crate) dma_isr_htif: DmaCh::CDmaIsrHtif,
    pub(crate) dma_isr_tcif: DmaCh::CDmaIsrTcif,
    pub(crate) dma_isr_teif: DmaCh::CDmaIsrTeif,
}

static DUMMY_U8:[u8; 1] = [0];

impl<DmaCh: DmaChMap> DmaChDiverged<DmaCh> {
    pub(crate) fn init_dma_rx(&self, per_dr: u32, chsel: u32, priority: u32) {
        self.dma_cpar.store_reg(|r, v| {
            r.pa().write(v, per_dr); // peripheral address
        });
        self.dma_ccr.store_reg(|r, v| {
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
    }

    pub(crate) fn init_dma_tx(&self, per_dr: u32, chsel: u32, priority: u32) {
        self.dma_cpar.store_reg(|r, v| {
            r.pa().write(v, per_dr); // peripheral address
        });
        self.dma_ccr.store_reg(|r, v| {
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
    }

    pub(crate) fn panic_on_err<DmaInt: IntToken>(&self, dma_int: DmaInt) {
        // Attach dma error handler
        let dma_isr_dmeif = self.dma_isr_dmeif;
        let dma_isr_feif = self.dma_isr_feif;
        let dma_isr_teif = self.dma_isr_teif;
        dma_int.add_fn(move || {
            // Load _entire_ interrupt status register.
            // The value is not masked to TEIF.
            let val = dma_isr_teif.load_val();
            handle_dma_err::<DmaCh>(&val, dma_isr_dmeif, dma_isr_feif, dma_isr_teif);
            fib::Yielded::<(), !>(())
        });
    }

    pub(crate) unsafe fn setup_stream(&self, buf: &[u8]) {
        // Memory address pointer is incremented after each data transfer
        self.dma_ccr.modify_reg(|r, v| {
            r.minc().set(v);
        });

        // Set buffer memory addres.
        self.dma_cm0ar.store_reg(|r, v| {
            r.m0a().write(v, buf.as_ptr() as u32);
        });

        // Set number of bytes to transfer.
        self.dma_cndtr.store_reg(|r, v| {
            r.ndt().write(v, buf.len() as u32);
        });

        // Clear transfer completed interrupt flag.
        self.dma_ifcr_ctcif.set_bit();

        // Enable stream.
        self.dma_ccr.modify_reg(|r, v| r.en().set(v));
    }

    pub(crate) unsafe fn setup_dummy_stream(&self, len: usize) {
        // Memory address pointer is fixed
        self.dma_ccr.modify_reg(|r, v| {
            r.minc().clear(v);
        });

        // Set buffer memory addres.
        self.dma_cm0ar.store_reg(|r, v| {
            r.m0a().write(v, DUMMY_U8.as_ptr() as u32);
        });

        // Set number of bytes to transfer.
        self.dma_cndtr.store_reg(|r, v| {
            r.ndt().write(v, len as u32);
        });

        // Clear transfer completed interrupt flag.
        self.dma_ifcr_ctcif.set_bit();

        // Enable stream.
        self.dma_ccr.modify_reg(|r, v| r.en().set(v));
    }
}

impl<DmaCh: DmaChMap> From<DmaChPeriph<DmaCh>> for DmaChDiverged<DmaCh> {
    fn from(periph: DmaChPeriph<DmaCh>) -> Self {
        let DmaChPeriph {
            dma_ccr,
            dma_cfcr,
            dma_cm0ar,
            dma_cm1ar,
            dma_cndtr,
            dma_cpar,
            dma_ifcr_cdmeif,
            dma_ifcr_cfeif,
            dma_ifcr_chtif,
            dma_ifcr_ctcif,
            dma_ifcr_cteif,
            dma_isr_dmeif,
            dma_isr_feif,
            dma_isr_htif,
            dma_isr_tcif,
            dma_isr_teif,
        } = periph;
        Self {
            dma_ccr,
            dma_cfcr,
            dma_cm0ar,
            dma_cm1ar,
            dma_cndtr: dma_cndtr.into_copy(),
            dma_cpar,
            dma_ifcr_cdmeif,
            dma_ifcr_cfeif,
            dma_ifcr_chtif,
            dma_ifcr_ctcif: dma_ifcr_ctcif.into_copy(),
            dma_ifcr_cteif,
            dma_isr_dmeif: dma_isr_dmeif.into_copy(),
            dma_isr_feif: dma_isr_feif.into_copy(),
            dma_isr_htif: dma_isr_htif.into_copy(),
            dma_isr_tcif: dma_isr_tcif.into_copy(),
            dma_isr_teif: dma_isr_teif.into_copy(),
        }
    }
}

fn handle_dma_err<DmaCh: DmaChMap>(
    val: &DmaCh::DmaIsrVal,
    dma_isr_dmeif: DmaCh::CDmaIsrDmeif,
    dma_isr_feif: DmaCh::CDmaIsrFeif,
    dma_isr_teif: DmaCh::CDmaIsrTeif,
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
