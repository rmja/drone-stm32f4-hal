use crate::{
    diverged::{DmaChDiverged, UartDiverged},
};
use drone_cortexm::{fib, reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::{
    dma::ch::{traits::*, DmaChMap},
    uart::{traits::*, UartMap},
};

pub struct RxGuard<'a, Uart: UartMap, DmaRx: DmaChMap, DmaRxInt: IntToken> {
    uart: &'a UartDiverged<Uart>,
    dma_rx: &'a DmaChDiverged<DmaRx>,
    dma_rx_int: &'a DmaRxInt,
    ring_buf: Box<[u8]>,
    first: usize,
}

impl<'a, Uart: UartMap, DmaRx: DmaChMap, DmaRxInt: IntToken>
    RxGuard<'a, Uart, DmaRx, DmaRxInt>
{
    pub(crate) fn new(
        uart: &'a UartDiverged<Uart>,
        dma_rx: &'a DmaChDiverged<DmaRx>,
        dma_rx_int: &'a DmaRxInt,
        buf: Box<[u8]>,
    ) -> Self {
        // Enable receiver.
        uart.uart_cr1.modify_reg(|r, v| {
            r.re().set(v);
        });
        
        let mut rx = Self {
            uart,
            dma_rx,
            dma_rx_int,
            ring_buf: buf,
            first: 0,
        };

        unsafe {
            rx.setup_dma();
        }

        rx
    }

    /// Read into buffer using DMA to the uart peripheral.
    pub async fn read(&mut self, buf: &mut [u8]) -> Result<usize, i32> {
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

        let ndtr = self.dma_rx.dma_cndtr.ndt().read_bits() as usize;
        let end = buf.len() - ndtr;

        let read = if self.first == end {
            // There currently no bytes available in the buffer.

            // Return a buffer overflow error if TCIF is asserted
            // as the DMA controller in that case has wrapped.
            if self.dma_rx.dma_isr_tcif.read_bit() {
                // Clear transfer completed interrupt flag.
                self.dma_rx.dma_ifcr_ctcif.set_bit();

                return Err(123);
            }

            // Listen for 
            // self.uart_cr1.modify_reg(|r, v| {
            //     r.tcie().set(v);
            // });

            0
        }
        else {
            // There are bytes readily available in the buffer.

            if self.first < end {
                // The available portion _does not_ wrap.

                // Return a buffer overflow error if TCIF is asserted
                // as the DMA controller in that case has wrapped.
                if self.dma_rx.dma_isr_tcif.read_bit() {
                    // Clear transfer completed interrupt flag.
                    self.dma_rx.dma_ifcr_ctcif.set_bit();

                    return Err(123);
                }

                let src = self.first..end;
                let dst = 0..buf.len();
                let cnt = core::cmp::min(src.len(), dst.len());
                buf[dst].copy_from_slice(&self.ring_buf[src]);
                self.first += cnt;

                cnt
            }
            else {
                // The available portion _does_ wrap.

                // Clear transfer completed interrupt flag.
                self.dma_rx.dma_ifcr_ctcif.set_bit();

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

                cnt_tail + cnt_head
            }
        };

        Ok(read)
    }

    unsafe fn setup_dma(&mut self) {
        // Set buffer memory addres
        self.dma_rx.dma_cm0ar.store_reg(|r, v| {
            r.m0a().write(v, self.ring_buf.as_ptr() as u32);
        });

        // Set number of bytes to transfer
        self.dma_rx.dma_cndtr.store_reg(|r, v| {
            r.ndt().write(v, self.ring_buf.len() as u32);
        });

        // Enable stream
        self.dma_rx.dma_ccr.modify_reg(|r, v| r.en().set(v));
    }
}

impl<Uart: UartMap, DmaRx: DmaChMap, DmaRxInt: IntToken> Drop
    for RxGuard<'_, Uart, DmaRx, DmaRxInt>
{
    fn drop(&mut self) {
        // Disable receiver.
        self.uart.uart_cr1.modify_reg(|r, v| {
            r.re().clear(v);
        });
    }
}