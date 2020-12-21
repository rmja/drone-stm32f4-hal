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
            10: pub exti4;
        }
    };
}
