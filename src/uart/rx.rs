use crate::{
    diverged::{DmaChDiverged, UartDiverged},
};
use drone_cortexm::{fib, reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::{
    dma::ch::{traits::*, DmaChMap},
    uart::{traits::*, UartMap},
};

pub struct UartRxDrv<'drv, Uart: UartMap, UartInt: IntToken, DmaRx: DmaChMap, DmaRxInt: IntToken> {
    pub(crate) uart: &'drv UartDiverged<Uart>,
    pub(crate) uart_int: &'drv UartInt,
    pub(crate) dma: DmaChDiverged<DmaRx>,
    pub(crate) dma_int: DmaRxInt,
}

pub struct RxGuard<'sess, Uart: UartMap, UartInt: IntToken, DmaRx: DmaChMap, DmaRxInt: IntToken> {
    drv: &'sess UartRxDrv<'sess, Uart, UartInt, DmaRx, DmaRxInt>,
    ring_buf: Box<[u8]>,
    first: usize,
    last_read_wrapped: bool,
}
#[derive(Copy, Clone)]
pub enum RxError {
    PossibleOverflow,
    Overflow,
}

impl<'drv, Uart: UartMap, UartInt: IntToken, DmaRx: DmaChMap, DmaRxInt: IntToken>
    UartRxDrv<'drv, Uart, UartInt, DmaRx, DmaRxInt>
{
    pub(crate) fn init_dma_rx(&mut self, channel: u32, priority: u32) {
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
            r.circ().set(v); // circular mode.
            r.dir().write(v, 0b00); // peripheral-to-memory
            r.tcie().clear(v); // transfer complete interrupt disable
            r.teie().set(v); // transfer error interrupt enable
        });

        // Attach dma error handler
        let dma_isr_dmeif = self.dma.dma_isr_dmeif;
        let dma_isr_feif = self.dma.dma_isr_feif;
        let dma_isr_teif = self.dma.dma_isr_teif;
        self.dma_int.add_fn(move || {
            // Load _entire_ interrupt status register.
            // The value is not masked to TEIF.
            let val = dma_isr_teif.load_val();
            crate::drv::handle_dma_err::<DmaRx>(&val, dma_isr_dmeif, dma_isr_feif, dma_isr_teif);
            fib::Yielded::<(), !>(())
        });
    }

    pub fn sess<'sess>(&'sess mut self, ring_buf: Box<[u8]>) -> RxGuard<'sess, Uart, UartInt, DmaRx, DmaRxInt> {
        // Enable receiver.
        self.uart.uart_cr1.modify_reg(|r, v| {
            r.re().set(v);
        });

        let mut rx = RxGuard {
            drv: self,
            ring_buf,
            first: 0,
            last_read_wrapped: false
        };

        unsafe {
            rx.setup_dma();
        }

        rx
    }
}

impl<'sess, Uart: UartMap, UartInt: IntToken, DmaRx: DmaChMap, DmaRxInt: IntToken>
    RxGuard<'sess, Uart, UartInt, DmaRx, DmaRxInt>
{
    /// Read into buffer using DMA to the uart peripheral.
    pub async fn read(&mut self, buf: &mut [u8]) -> Result<usize, RxError> {
        let drv = self.drv;
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

        let ndtr = drv.dma.dma_cndtr.ndt().read_bits() as usize;
        let mut end = buf.len() - ndtr;

        if self.first == end {
            // There currently no bytes available in the buffer.

            // Return a buffer overflow error if TCIF is asserted
            // as the DMA controller in that case has wrapped.
            // This is the special case where n*ring_buf.len(), n > 0, bytes have been written since last read.
            if drv.dma.dma_isr_tcif.read_bit() {
                // Clear transfer completed interrupt flag.
                drv.dma.dma_ifcr_ctcif.set_bit();

                return Err(RxError::Overflow);
            }

            let dma_cndtr = self.drv.dma.dma_cndtr;
            let any_rx = drv.uart_int.add_future(fib::new_fn(move || {
                // Note that we cannot clear the RXNE flag as it is automatically cleared by the DMA controller.
                let new_ndtr = dma_cndtr.ndt().read_bits() as usize;
                if new_ndtr != ndtr {
                    fib::Complete(new_ndtr)
                } else {
                    fib::Yielded(())
                }
            }));

            // Listen for any rx activity.
            drv.uart.uart_cr1.modify_reg(|r, v| {
                r.rxneie().set(v);
            });

            let mut new_ndtr = drv.dma.dma_cndtr.ndt().read_bits() as usize;
            if new_ndtr == ndtr {
                // Wait for actitivy.
                new_ndtr = any_rx.await;
            }

            // Stop listen activity.
            drv.uart.uart_cr1.modify_reg(|r, v| {
                r.rxneie().clear(v);
            });

            end = buf.len() - new_ndtr;
        }

        // There are bytes readily available in the buffer at this time.

        if self.first < end {
            // The available portion _does not_ wrap.

            // Return a buffer overflow error if TCIF is asserted
            // as the DMA controller in that case has wrapped.
            if drv.dma.dma_isr_tcif.read_bit() {
                // Clear transfer completed interrupt flag.
                drv.dma.dma_ifcr_ctcif.set_bit();
                return Err(RxError::Overflow);
            }
            self.last_read_wrapped = false;

            let src = self.first..end;
            let dst = 0..buf.len();
            let cnt = core::cmp::min(src.len(), dst.len());
            buf[dst].copy_from_slice(&self.ring_buf[src]);
            self.first += cnt;

            Ok(cnt)
        }
        else {
            // The available portion _does_ wrap.

            // Clear transfer completed interrupt flag.
            drv.dma.dma_ifcr_ctcif.set_bit();
            if self.last_read_wrapped {
                return Err(RxError::PossibleOverflow);
            }
            self.last_read_wrapped = true;

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

            Ok(cnt_tail + cnt_head)
        }
    }

    unsafe fn setup_dma(&mut self) {
        let drv = self.drv;

        // Set buffer memory addres
        drv.dma.dma_cm0ar.store_reg(|r, v| {
            r.m0a().write(v, self.ring_buf.as_ptr() as u32);
        });

        // Set number of bytes to transfer
        drv.dma.dma_cndtr.store_reg(|r, v| {
            r.ndt().write(v, self.ring_buf.len() as u32);
        });

        // Enable stream
        drv.dma.dma_ccr.modify_reg(|r, v| r.en().set(v));
    }
}

impl<Uart: UartMap, UartInt: IntToken, DmaRx: DmaChMap, DmaRxInt: IntToken> Drop
    for RxGuard<'_, Uart, UartInt, DmaRx, DmaRxInt>
{
    /// Stop the receiver.
    fn drop(&mut self) {
        // Disable receiver.
        self.drv.uart.uart_cr1.modify_reg(|r, v| {
            r.re().clear(v);
        });
    }
}