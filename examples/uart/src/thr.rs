//! The threads.

pub use drone_cortexm::thr::{init, init_extended};
pub use drone_stm32_map::thr::*;

use drone_cortexm::thr;

thr! {
    /// The thread data.
    thread => pub Thr {};

    /// The thread-local storage.
    local => pub ThrLocal {};

    /// The vector table type.
    vtable => pub Vtable;

    /// A set of thread tokens.
    index => pub Thrs;

    /// Threads initialization token.
    init => pub ThrsInit;

    threads => {
        exceptions => {
            /// All classes of faults.
            pub hard_fault;
        };
        interrupts => {
            // Vector table for stm32f429 is in PM0090 table 62 page 375.
            5: pub rcc;
            // 12: pub dma1_ch1; // USART3_RX: DMA1, stream 1 (channel 4).
            // 14: pub dma1_ch3; // USART3_TX: DMA1, stream 3 (channel 4).
            16: pub dma1_ch5; // USART2_RX: DMA1, stream 5 (channel 4).
            17: pub dma1_ch6; // USART2_TX: DMA1, stream 6 (channel 4).

            38: pub usart2;
            // 39: pub usart3;
        }
    };
}
