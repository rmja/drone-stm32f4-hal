use crate::{
    diverged::{DmaChDiverged, UartDiverged},
};
use core::ops::Range;
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
            crate::drv::handle_dma_err::<DmaRx>(&val, dma_isr_dmeif, dma_isr_feif, dma_isr_teif);
            fib::Yielded::<(), !>(())
        });
    }

    /// Enable rx operation for the uart peripheral and return a guard that disables the receiver when dropped.
    /// Bytes are received into `ring_buf` and `read()` calls must be made in a sufficent pace to keep up with the receiption.
    /// `read()' calls must always keep the ring buffer less than half full for the driver to correctly detect if overflows have occured.
    pub fn sess<'sess>(&'sess mut self, ring_buf: Box<[u8]>) -> RxGuard<'sess, Uart, UartInt, DmaRx, DmaRxInt> {
        let mut rx = RxGuard {
            drv: self,
            ring_buf,
            first: 0,
            last_read_wrapped: false
        };

        rx.start();

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
        //  + ring_buf                 +--- NDTR ---+       + ring_buf   +---------- NDTR ----------+
        //  |                          |            |       |            |                          |
        //  v                          v            v       v            v                          v
        // +-----------------------------------------+     +-----------------------------------------+
        // |oooooooooooXXXXXXXXXXXXXXXXoooooooooooooo|     |XXXXXXXXXXXXXooooooooooooXXXXXXXXXXXXXXXX|
        // +-----------------------------------------+     +-----------------------------------------+
        //  ^          ^               ^                    ^            ^           ^
        //  |          |               |                    |            |           |
        //  +- first --+               |                    +- end ------+           |
        //  |                          |                    |                        |
        //  +- end --------------------+                    +- first ----------------+


        // NDTR is auto-reloaded to the ring buffer size when it reaches 0.
        // The transfer completed interrupt flag (TCIF) is asserted when this happens,
        // which is used to detect overflows in the ring buffer.

        let mut ndtr = drv.dma.dma_cndtr.ndt().read_bits() as usize;
        let mut end = self.ring_buf.len() - ndtr;

        if self.first == end {
            // There currently no bytes readily available in the buffer.

            // Return a buffer overflow error if TCIF is asserted
            // as the DMA controller in that case has wrapped.
            // This is the special case where n*ring_buf.len(), n > 0,1,2,..., bytes have been written since last read.
            if drv.dma.dma_isr_tcif.read_bit() {
                // Clear transfer completed interrupt flag.
                drv.dma.dma_ifcr_ctcif.set_bit();

                return Err(RxError::Overflow);
            }

            // Wait for any number of bytes to arrive in the rx ring buffer.
            self.any_rx(ndtr).await;

            // Update the ring buffer values to new values after some bytes have been received.
            ndtr = drv.dma.dma_cndtr.ndt().read_bits() as usize;
            end = self.ring_buf.len() - ndtr;
        }

        // There are at this time bytes readily available in the ring buffer.

        if self.first < end {
            // The available portion in the ring buffer _does not_ wrap.

            // Return a buffer overflow error if TCIF is asserted
            // as the DMA controller in that case has wrapped.
            if drv.dma.dma_isr_tcif.read_bit() {
                // Clear transfer completed interrupt flag.
                drv.dma.dma_ifcr_ctcif.set_bit();
                return Err(RxError::Overflow);
            }
            self.last_read_wrapped = false;

            let cnt = self.copy_to(buf, self.first..end);
            self.first = (self.first + cnt) % self.ring_buf.len();

            Ok(cnt)
        }
        else {
            // The available portion in the ring buffer _does_ wrap.

            if self.first + buf.len() < self.ring_buf.len() {
                // The dma controller has wrapped and is currently writing (or the next byte added will be) in the beginning of the ring buffer,
                // but the provided read buffer is not sufficiently large to empty the tail in the ring buffer.

                // Return a buffer overflow error if TCIF is asserted
                // as the DMA controller in that case has wrapped.
                if drv.dma.dma_isr_tcif.read_bit() {
                    // Clear transfer completed interrupt flag.
                    drv.dma.dma_ifcr_ctcif.set_bit();
                    return Err(RxError::Overflow);
                }
                self.last_read_wrapped = false;

                let cnt = self.copy_to(buf, self.first..self.ring_buf.len());
                self.first = (self.first + cnt) % self.ring_buf.len();

                Ok(cnt)
            }
            else {
                // The provided read buffer is large enough to include all bytes from the tail of the ring buffer,
                // so that the next read will start from the beginning.

                // Clear transfer completed interrupt flag.
                drv.dma.dma_ifcr_ctcif.set_bit();
                if self.last_read_wrapped {
                    return Err(RxError::PossibleOverflow);
                }
                self.last_read_wrapped = true;

                let cnt_tail = self.copy_to(buf, self.first..self.ring_buf.len());
                let cnt_head = self.copy_to(&mut buf[..cnt_tail], 0..end);
                self.first = cnt_head;

                Ok(cnt_tail + cnt_head)
            }
        }
    }

    fn copy_to(&mut self, buf: &mut [u8], data_range: Range<usize>) -> usize {
        // Limit the number of bytes that can be copied.
        let cnt = core::cmp::min(data_range.len(), buf.len());
        let data_range = limit(data_range, cnt);

        // Copy from ring buffer into read buffer.
        buf[0..cnt].copy_from_slice(&self.ring_buf[data_range]);

        cnt
    }

    fn start(&mut self) {
        let drv = self.drv;

        unsafe {
            self.setup_dma();
        }

        // Enable receiver.
        drv.uart.uart_cr1.modify_reg(|r, v| {
            r.re().set(v);
        });

        // Start receive on DMA channel.
        drv.uart.uart_cr3.modify_reg(|r, v| {
            r.dmar().set(v);
        });
    }

    fn stop(&mut self) {
        let drv = self.drv;

        // Stop receive on DMA channel.
        drv.uart.uart_cr3.modify_reg(|r, v| {
            r.dmar().clear(v);
        });

        // Disable receiver.
        drv.uart.uart_cr1.modify_reg(|r, v| {
            r.re().clear(v);
        });

        // Disable dma stream.
        drv.dma.dma_ccr.modify_reg(|r, v| r.en().clear(v));
    }

    unsafe fn setup_dma(&mut self) {
        let drv = self.drv;

        // Set buffer memory addres.
        drv.dma.dma_cm0ar.store_reg(|r, v| {
            r.m0a().write(v, self.ring_buf.as_ptr() as u32);
        });

        // Set number of bytes to transfer.
        drv.dma.dma_cndtr.store_reg(|r, v| {
            r.ndt().write(v, self.ring_buf.len() as u32);
        });

        // Clear transfer completed interrupt flag.
        drv.dma.dma_ifcr_ctcif.set_bit();

        // Enable stream.
        drv.dma.dma_ccr.modify_reg(|r, v| r.en().set(v));
    }

    async fn any_rx(&mut self, old_ndtr: usize) {
        let drv = self.drv;
        let dma_cndtr = drv.dma.dma_cndtr;
        let any_rx = drv.uart_int.add_future(fib::new_fn(move || {
            // Note that we cannot clear the RXNE flag as it is automatically cleared by the DMA controller.
            let new_ndtr = dma_cndtr.ndt().read_bits() as usize;
            if new_ndtr != old_ndtr {
                fib::Complete(())
            } else {
                fib::Yielded(())
            }
        }));

        // Listen for any rx activity.
        drv.uart.uart_cr1.modify_reg(|r, v| {
            r.rxneie().set(v);
        });

        let new_ndtr = drv.dma.dma_cndtr.ndt().read_bits() as usize;
        if new_ndtr == old_ndtr {
            // Wait for actitivy.
            any_rx.await;
        }

        // Stop listen for activity.
        drv.uart.uart_cr1.modify_reg(|r, v| {
            r.rxneie().clear(v);
        });
    }
}

impl<Uart: UartMap, UartInt: IntToken, DmaRx: DmaChMap, DmaRxInt: IntToken> Drop
    for RxGuard<'_, Uart, UartInt, DmaRx, DmaRxInt>
{
    /// Stop the receiver.
    fn drop(&mut self) {
        self.stop();
    }
}

fn limit(range: Range<usize>, limit: usize) -> Range<usize> {
    if range.len() <= limit {
        range
    }
    else {
        Range { start: range.start, end: range.start + limit }
    }
}