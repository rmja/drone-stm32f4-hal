//! The threads.

drone_cortexm::thr::nvic! {
    /// Thread-safe storage.
    thread => pub Thr {};

    /// Thread-local storage.
    local => pub Local {};

    /// Collection of exception vectors.
    vectors => pub Vectors;

    /// Vector table type.
    vtable => pub Vtable;

    /// Thread token set.
    index => pub Index;

    /// Threads initialization token.
    init => pub Init;

    threads => {
        exceptions => {
            /// All classes of faults.
            pub hard_fault;
            ////// Add additional exception handlers like SYS_TICK.
            // /// System tick timer.
            // pub sys_tick;
        };
        ////// Add interrupt handlers. The name for the handler is arbitrary,
        ////// and the number should correspond to the hardware NVIC interrupt
        ////// number.
        interrupts => {
            // Vector table for stm32f429 is in PM0090 table 62 page 375.
            // /// RCC global interrupt.
            // 5: pub rcc;
            8: pub exti2;
            // 10: pub exti4;
        };
    };
}
