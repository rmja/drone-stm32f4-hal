use core::marker::PhantomData;

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f410",
    stm32_mcu = "stm32f411",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f415",
    stm32_mcu = "stm32f417",
    stm32_mcu = "stm32f423",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f437",
    stm32_mcu = "stm32f439",
    stm32_mcu = "stm32f469",
    stm32_mcu = "stm32f479",
))]
/// Minimum CPU clock frequency.
pub const SYSCLK_MIN: u32 = 24_000_000;

#[cfg(any(stm32_mcu = "stm32f446",))]
/// Minimum CPU clock frequency.
pub const SYSCLK_MIN: u32 = 12_500_000;

#[cfg(any(stm32_mcu = "stm32f401",))]
/// Maximum CPU clock frequency.
pub const SYSCLK_MAX: u32 = 84_000_000;

#[cfg(any(
    stm32_mcu = "stm32f410",
    stm32_mcu = "stm32f411",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f423",
))]
/// Maximum CPU clock frequency.
pub const SYSCLK_MAX: u32 = 100_000_000;

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f415",
    stm32_mcu = "stm32f417",
))]
/// Maximum CPU clock frequency.
pub const SYSCLK_MAX: u32 = 168_000_000;

#[cfg(any(
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f437",
    stm32_mcu = "stm32f439",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
    stm32_mcu = "stm32f479",
))]
/// Maximum CPU clock frequency.
pub const SYSCLK_MAX: u32 = 180_000_000;

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f410",
    stm32_mcu = "stm32f411",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f423",
))]
/// Maximum APB2 peripheral clock frequency.
pub const PCLK2_MAX: u32 = SYSCLK_MAX;

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f415",
    stm32_mcu = "stm32f417",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f437",
    stm32_mcu = "stm32f439",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
    stm32_mcu = "stm32f479",
))]
/// Maximum APB2, high speed peripheral clock frequency.
pub const PCLK2_MAX: u32 = SYSCLK_MAX / 2;

/// Maximum APB1, low speed peripheral clock frequency.
pub const PCLK1_MAX: u32 = PCLK2_MAX / 2;

pub trait Freq {
    /// Get the clock frequency.
    fn freq(&self) -> u32;
}

/// A peripheral clock token.
pub trait PClkToken: Freq {}

// Clock source selector
pub struct Mux<Signal> {
    _signal: PhantomData<Signal>,
}

/// The High-Speed External (HSE) clock.
#[derive(Copy, Clone)]
pub struct HseClk(u32);

impl HseClk {
    pub const fn new(freq: u32) -> HseClk {
        assert!(freq >= 4_000_000 && freq <= 26_000_000);
        HseClk(freq)
    }

    pub const fn f(&self) -> u32 {
        self.0
    }
}

impl Freq for HseClk {
    fn freq(&self) -> u32 {
        self.f()
    }
}

/// The High-Speed Internal (HSI) 16MHz clock.
#[derive(Copy, Clone)]
pub struct HsiClk;

impl HsiClk {
    pub const fn f(&self) -> u32 {
        16_000_000
    }
}

impl Freq for HsiClk {
    fn freq(&self) -> u32 {
        self.f()
    }
}

/// The pll clock source signal.
#[derive(Copy, Clone)]
pub enum PllSrcMuxSignal {
    Hsi(HsiClk),
    Hse(HseClk),
}

/// The pll clock source mux.
pub const PLLSRC_MUX: Mux<PllSrcMuxSignal> = Mux {
    _signal: PhantomData,
};

impl PllSrcMuxSignal {
    pub const fn f(&self) -> u32 {
        match self {
            PllSrcMuxSignal::Hsi(clk) => clk.f(),
            PllSrcMuxSignal::Hse(clk) => clk.f(),
        }
    }

    pub const fn to_pllsrc(self, pll_m: u32) -> PllSrc {
        PllSrc::new(self, pll_m)
    }
}

impl Freq for PllSrcMuxSignal {
    fn freq(&self) -> u32 {
        self.f()
    }
}

/// The pll input clock (vcoin).
#[derive(Copy, Clone)]
pub struct PllSrc {
    /// The pll input clock source.
    pub mux: PllSrcMuxSignal,
    /// Pll source clock input division factor.
    /// vcoin = src / m
    pub m: u32,
}

impl PllSrc {
    #[must_use]
    const fn new(mux: PllSrcMuxSignal, m: u32) -> PllSrc {
        assert!(m >= 2 && m <= 63);
        PllSrc { mux, m }
    }

    pub const fn f(&self) -> u32 {
        self.mux.f() / self.m
    }

    pub const fn to_pll(self, pll_n: u32, pll_p: u32, pll_q: u32) -> Pll {
        let vco = PllVco::new(self, pll_n);
        Pll::new(vco, pll_p, pll_q)
    }
}

impl Freq for PllSrc {
    fn freq(&self) -> u32 {
        self.f()
    }
}

/// The clocks generated by the pll.
/// pllp: pllclk = vcoin * n / p
/// pllq: vcoin * n / q
#[derive(Copy, Clone)]
pub struct PllVco {
    /// The pll input clock signal (vcoin).
    pub src: PllSrc,
    /// Pll multiplication factor for vco.
    /// vccout = vcoin * n
    pub n: u32,
}

impl PllVco {
    #[must_use]
    const fn new(src: PllSrc, n: u32) -> PllVco {
        assert!(n >= 50 && n <= 432);
        PllVco { src, n }
    }

    pub const fn f(&self) -> u32 {
        self.src.f() * self.n
    }
}

impl Freq for PllVco {
    fn freq(&self) -> u32 {
        self.f()
    }
}

#[derive(Copy, Clone)]
pub struct PllP;
#[derive(Copy, Clone)]
pub struct PllQ;

/// A pll generated clock, e.g. PllClk<PllP> = vcoin * n / p
#[derive(Copy, Clone)]
pub struct PllClk<Out> {
    _out: PhantomData<Out>,
    src: PllVco,
    pub div: u32,
}

impl<Out> PllClk<Out> {
    pub const fn f(&self) -> u32 {
        self.src.f() / self.div
    }
}

impl<Out> Freq for PllClk<Out> {
    fn freq(&self) -> u32 {
        self.f()
    }
}

#[derive(Copy, Clone)]
pub struct Pll {
    pub vco: PllVco,
    /// Pll division factor for system clock.
    pub p: PllClk<PllP>,
    /// Pll division factor for usb, sdio, and rng.
    pub q: PllClk<PllQ>,
}

impl Pll {
    #[must_use]
    const fn new(vco: PllVco, p: u32, q: u32) -> Pll {
        assert!(p == 2 || p == 4 || p == 6 || p == 8);
        assert!(q >= 2 && q <= 15);
        let pll = Pll {
            vco,
            p: PllClk {
                _out: PhantomData,
                src: vco,
                div: p,
            },
            q: PllClk {
                _out: PhantomData,
                src: vco,
                div: q,
            },
        };
        assert!(pll.q.f() <= 48_000_000);
        pll
    }
}

/// The system clock source mux.

/// The system clock source signal.
#[derive(Copy, Clone)]
pub enum SysClkMuxSignal {
    Hsi(HsiClk),
    Hse(HseClk),
    Pll(PllClk<PllP>),
}

pub const SYSCLK_MUX: Mux<SysClkMuxSignal> = Mux {
    _signal: PhantomData,
};

impl SysClkMuxSignal {
    pub const fn f(&self) -> u32 {
        match self {
            SysClkMuxSignal::Hsi(clk) => clk.f(),
            SysClkMuxSignal::Hse(clk) => clk.f(),
            SysClkMuxSignal::Pll(clk) => clk.f(),
        }
    }

    pub const fn to_sysclk(self) -> SysClk {
        SysClk::new(self)
    }
}

impl Freq for SysClkMuxSignal {
    fn freq(&self) -> u32 {
        self.f()
    }
}

/// The sysclk (CPU clock).
#[derive(Copy, Clone)]
pub struct SysClk {
    /// The clock source.
    mux: SysClkMuxSignal,
}

impl SysClk {
    #[must_use]
    const fn new(mux: SysClkMuxSignal) -> SysClk {
        let sysclk = SysClk { mux };
        let f = sysclk.f();
        assert!(f >= SYSCLK_MIN && f <= SYSCLK_MAX);
        sysclk
    }

    pub const fn f(&self) -> u32 {
        self.mux.f()
    }

    pub const fn to_hclk(self, hpre: u32) -> HClk {
        assert!(
            hpre == 1
                || hpre == 2
                || hpre == 4
                || hpre == 8
                || hpre == 16
                || hpre == 64
                || hpre == 128
                || hpre == 256
                || hpre == 512
        );
        HClk { src: self, hpre }
    }
}

/// The AHB (Advanced High-Performance Bus) and CPU clock.
#[derive(Copy, Clone)]
pub struct HClk {
    /// The clock source.
    src: SysClk,
    /// The clock prescaler.
    /// hclk = sysclk / hpre
    pub hpre: u32,
}

impl HClk {
    pub const fn f(&self) -> u32 {
        self.src.f() / self.hpre
    }

    pub const fn to_pclk1(self, ppre1: u32) -> PClk1 {
        PClk1::new(self, ppre1)
    }

    pub const fn to_pclk2(self, ppre2: u32) -> PClk2 {
        PClk2::new(self, ppre2)
    }

    pub const fn to_systickclk(self) -> SysTickClk {
        SysTickClk{ src: self }
    }
}

impl Freq for HClk {
    fn freq(&self) -> u32 {
        self.f()
    }
}

/// The APB1 (Low Speed Advanced Peripheral Bus) peripheral clock.
#[derive(Copy, Clone)]
pub struct PClk1 {
    /// The clock source.
    src: HClk,
    /// The clock prescaler.
    /// pclk1 = hclk / ppre1
    pub ppre1: u32,
}

impl PClk1 {
    #[must_use]
    const fn new(src: HClk, ppre1: u32) -> PClk1 {
        assert!(ppre1 == 1 || ppre1 == 2 || ppre1 == 4 || ppre1 == 8 || ppre1 == 16);
        let pclk1 = PClk1 { src, ppre1 };
        assert!(pclk1.f() <= PCLK1_MAX);
        pclk1
    }

    pub const fn f(&self) -> u32 {
        self.src.f() / self.ppre1
    }
}

impl PClkToken for PClk1 {}

impl Freq for PClk1 {
    fn freq(&self) -> u32 {
        self.f()
    }
}

/// The APB2 (High Speed Advanced Peripheral Bus) peripheral clock.
#[derive(Copy, Clone)]
pub struct PClk2 {
    src: HClk,
    /// The clock prescaler.
    /// pclk2 = hclk / ppre2
    pub ppre2: u32,
}

impl PClkToken for PClk2 {}

impl PClk2 {
    #[must_use]
    const fn new(src: HClk, ppre2: u32) -> PClk2 {
        assert!(ppre2 == 1 || ppre2 == 2 || ppre2 == 4 || ppre2 == 8 || ppre2 == 16);
        let pclk2 = PClk2 { src, ppre2 };
        assert!(pclk2.f() <= PCLK2_MAX);
        pclk2
    }

    pub const fn f(&self) -> u32 {
        self.src.f() / self.ppre2
    }
}

impl Freq for PClk2 {
    fn freq(&self) -> u32 {
        self.f()
    }
}

/// The Cortex System Timer Clock.
#[derive(Copy, Clone)]
pub struct SysTickClk {
    src: HClk,
}

impl SysTickClk {
    pub const fn f(&self) -> u32 {
        self.src.f() / 8
    }
}

impl Freq for SysTickClk {
    fn freq(&self) -> u32 {
        self.f()
    }
}