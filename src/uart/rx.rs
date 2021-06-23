use crate::diverged::{DmaChDiverged, UartDiverged};
use alloc::sync::Arc;
use core::ops::Range;
use drone_cortexm::{fib, reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::{
    dma::ch::{traits::*, DmaChMap},
    uart::{traits::*, UartMap},
};
use drone_stm32f4_dma_drv::{DmaChCfg, DmaStChToken};

pub struct UartRxDrv<Uart: UartMap, UartInt: IntToken, DmaRx: DmaChMap> {
    pub(crate) uart: Arc<UartDiverged<Uart>>,
    pub(crate) uart_int: UartInt,
    pub(crate) dma: DmaChDiverged<DmaRx>,
}

pub struct RxGuard<'sess, Uart: UartMap, UartInt: IntToken, DmaRx: DmaChMap> {
    drv: &'sess UartRxDrv<Uart, UartInt, DmaRx>,
    ring_buf: Box<[u8]>,
    first: usize,
    last_read_wrapped: bool,
}
#[derive(Copy, Clone, Debug)]
pub enum RxError {
    PossibleOverflow,
    Overflow,
}

impl<Uart: UartMap, UartInt: IntToken, DmaRx: DmaChMap>
    UartRxDrv<Uart, UartInt, DmaRx>
{
    pub(crate) fn init<DmaRxStCh: DmaStChToken, DmaRxInt: IntToken>(
        uart: Arc<UartDiverged<Uart>>,
        uart_int: UartInt,
        rx_cfg: DmaChCfg<DmaRx, DmaRxStCh, DmaRxInt>,
    ) -> Self {
        let DmaChCfg {
            dma_ch,
            dma_int,
            dma_pl,
            ..
        } = rx_cfg;
        let rx = Self {
            uart,
            uart_int,
            dma: dma_ch.into(),
        };
        rx.dma
            .init_dma_rx(rx.uart.uart_dr.as_mut_ptr() as u32, DmaRxStCh::NUM, dma_pl);
        rx.dma.panic_on_err(dma_int);
        rx
    }

    /// Enable rx operation for the uart peripheral and return a guard that disables the receiver when dropped.
    /// Bytes are received into `ring_buf` and `read()` calls must be made in a sufficent pace to keep up with the reception.
    /// `read()' calls must always keep the ring buffer less than half full for the driver to correctly detect if overflows have occured.
    pub fn start(&mut self, ring_buf: Box<[u8]>) -> RxGuard<Uart, UartInt, DmaRx> {
        let mut rx = RxGuard {
            drv: self,
            ring_buf,
            first: 0,
            last_read_wrapped: false,
        };
        rx.start();
        rx
    }
}

impl<'sess, Uart: UartMap, UartInt: IntToken, DmaRx: DmaChMap>
    RxGuard<'sess, Uart, UartInt, DmaRx>
{
    /// Read from the rx ring buffer into `buf`.
    /// Wait for any receiption if no bytes are readily awailable in the ring buffer.
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
            self.any_rx_activity(ndtr).await;

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
        } else {
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
            } else {
                // The provided read buffer is large enough to include all bytes from the tail of the ring buffer,
                // so the next read will not have any unread tail bytes in the ring buffer.

                // Clear transfer completed interrupt flag.
                drv.dma.dma_ifcr_ctcif.set_bit();
                if self.last_read_wrapped {
                    return Err(RxError::PossibleOverflow);
                }
                self.last_read_wrapped = true;

                let cnt_tail = self.copy_to(buf, self.first..self.ring_buf.len());
                let cnt_head = self.copy_to(&mut buf[cnt_tail..], 0..end);
                self.first = cnt_head;

                Ok(cnt_tail + cnt_head)
            }
        }
    }

    /// Copy from the rx ring buffer at `data_range` into `buf`.
    fn copy_to(&mut self, buf: &mut [u8], data_range: Range<usize>) -> usize {
        // Limit the number of bytes that can be copied.
        let cnt = core::cmp::min(data_range.len(), buf.len());
        let data_range = limit(data_range, cnt);

        // Copy from ring buffer into read buffer.
        buf[0..cnt].copy_from_slice(&self.ring_buf[data_range]);

        cnt
    }

    /// Start the uart and dma according to AN4031 §4.3.
    fn start(&mut self) {
        let drv = self.drv;

        // 1-2. Configure the dma stream and enable it.
        unsafe {
            drv.dma.setup_stream(self.ring_buf.as_ref());
        }

        // 3a. Configure uart to receive on DMA channel.
        drv.uart.uart_cr3.modify_reg(|r, v| {
            r.dmar().set(v);
        });

        // 3b. Enable receiver peripheral.
        drv.uart.uart_cr1.modify_reg(|r, v| {
            r.re().set(v);
        });
    }

    /// Stop the uart and dma according to AN4031 §4.1.
    fn stop(&mut self) {
        let drv = self.drv;

        // 1. Disable dma stream.
        drv.dma.dma_ccr.modify_reg(|r, v| r.en().clear(v));

        // 2. Wait until the EN bit in DMA_SxCR register is reset.
        loop {
            if !drv.dma.dma_ccr.en().read_bit() {
                break;
            }
        }

        // 3a. Disable receiver.
        drv.uart.uart_cr1.modify_reg(|r, v| {
            r.re().clear(v);
        });

        // 3b. Stop receive on DMA channel.
        drv.uart.uart_cr3.modify_reg(|r, v| {
            r.dmar().clear(v);
        });
    }

    async fn any_rx_activity(&mut self, old_ndtr: usize) {
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

impl<Uart: UartMap, UartInt: IntToken, DmaRx: DmaChMap> Drop for RxGuard<'_, Uart, UartInt, DmaRx> {
    /// Stop the receiver.
    fn drop(&mut self) {
        self.stop();
    }
}

fn limit(range: Range<usize>, limit: usize) -> Range<usize> {
    if range.len() <= limit {
        range
    } else {
        Range {
            start: range.start,
            end: range.start + limit,
        }
    }
}
