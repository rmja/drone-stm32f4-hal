pub struct ExtiLine<
    'drv,
    Exti: ExtiMap + SyscfgExticrExti + ExtiRtsrRt + ExtiFtsrFt + ExtiSwierSwi + ExtiPrPif,
    ExtiInt: IntToken,
> {
    exti: &'drv ExtiDiverged<Exti>,
    exti_int: ExtiInt,
}

impl<
        'drv,
        Exti: ExtiMap + SyscfgExticrExti + ExtiRtsrRt + ExtiFtsrFt + ExtiSwierSwi + ExtiPrPif,
        ExtiInt: IntToken,
    > ExtiLine<'drv, Exti, ExtiInt>
{
    pub(crate) fn init<Edge: EdgeToken>(exti: &'drv ExtiDrv<Exti, ExtiInt, Edge>) -> Self {
        // self.exti.syscfg_exticr_exti.write_bits(config); // configuration
        Self {
            exti: &exti.exti,
            exti_int: exti.exti_int,
        }
    }

    /// Adds the fiber `fib` to the fiber chain running of the exti thread and returns a future, which
    /// resolves on completion of the fiber, i.e. when triggered.
    fn add_future<F, Y, T>(self, fib: F) -> FiberFuture<T>
    where
        F: Fiber<Input = (), Yield = Y, Return = T>,
        Y: YieldNone,
        F: Send + 'static,
        T: Send + 'static,
    {
        self.exti_int.add_future(fib)
    }

    /// Adds the fiber `fib` to the fiber chain running of the exti thread and returns a future, which
    /// resolves on completion of the fiber, i.e. when triggered.
    fn when_triggered(self) -> FiberFuture<()> {
        let exti_pr_pif = self.exti.exti_pr_pif;
        self.exti_int.add_future(fib::new_fn(move || {
            if exti_pr_pif.read_bit() {
                // Selected trigger request occurred, clear interrupt flag
                exti_pr_pif.set_bit();
                fib::Complete(())
            } else {
                fib::Yielded(())
            }
        }))
    }

    /// Creates a new saturating stream of external events.
    pub fn create_saturating_stream(&self) -> impl Stream<Item = NonZeroUsize> + Send + Sync {
        self.exti_int.add_saturating_pulse_stream(self.new_fib())
    }

    /// Creates a new fallible stream of external events.
    pub fn create_try_stream(
        &self,
    ) -> impl Stream<Item = Result<NonZeroUsize, ExtiOverflow>> + Send + Sync {
        self.exti_int
            .add_pulse_try_stream(|| Err(ExtiOverflow), self.new_fib())
    }

    fn new_fib<R>(&self) -> impl Fiber<Input = (), Yield = Option<usize>, Return = R> {
        let exti_pr_pif = self.exti.exti_pr_pif;
        fib::new_fn(move || {
            if exti_pr_pif.read_bit() {
                // selected trigger request occurred
                exti_pr_pif.set_bit();
                fib::Yielded(Some(1))
            } else {
                fib::Yielded(None)
            }
        })
    }
}
