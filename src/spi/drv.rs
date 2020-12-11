use crate::{diverged::SpiDiverged, master::SpiMasterDrv};
use config::{BaudRate, ClkPol, FirstBit};
use core::marker::PhantomData;
use drone_cortexm::{fib, reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::{
    dma::ch::DmaChMap,
    gpio::pin::GpioPinMap,
    spi::{traits::*, SpiCr1, SpiMap, SpiPeriph},
};
use drone_stm32f4_dma_drv::{DmaChCfg, DmaStCh0, DmaStCh3, DmaStChToken};
use drone_stm32f4_gpio_drv::{prelude::*, GpioPinCfg};
use drone_stm32f4_rcc_drv::{clktree::*, traits::ConfiguredClk};

pub mod config {
    use super::*;

    pub struct SpiPins<
        ClkPin: GpioPinMap,
        ClkType: PinTypeToken,
        ClkPull: PinPullToken,
        MisoPin: GpioPinMap,
        MisoType: PinTypeToken,
        MisoPull: PinPullToken,
        MosiPin: GpioPinMap,
        MosiType: PinTypeToken,
        MosiPull: PinPullToken,
        Af: PinAfToken,
    > {
        pub pin_clk: GpioPinCfg<ClkPin, AlternateMode<Af>, ClkType, ClkPull>,
        pub pin_miso: GpioPinCfg<MisoPin, AlternateMode<Af>, MisoType, MisoPull>,
        pub pin_mosi: GpioPinCfg<MosiPin, AlternateMode<Af>, MosiType, MosiPull>,
    }

    pub struct SpiSetup<
        Spi: SpiMap + SpiCr1,
        SpiInt: IntToken,
        ClkPin: GpioPinMap,
        ClkType: PinTypeToken,
        ClkPull: PinPullToken,
        MisoPin: GpioPinMap,
        MisoType: PinTypeToken,
        MisoPull: PinPullToken,
        MosiPin: GpioPinMap,
        MosiType: PinTypeToken,
        MosiPull: PinPullToken,
        Af: PinAfToken,
        Clk: PClkToken,
    > {
        /// Spi peripheral.
        pub spi: SpiPeriph<Spi>,
        /// Spi global interrupt.
        pub spi_int: SpiInt,
        /// Spi pins.
        pub spi_pins: SpiPins<
            ClkPin,
            ClkType,
            ClkPull,
            MisoPin,
            MisoType,
            MisoPull,
            MosiPin,
            MosiType,
            MosiPull,
            Af,
        >,
        /// Spi clock.
        pub clk: ConfiguredClk<Clk>,
        /// The baud rate.
        pub baud_rate: BaudRate,
        /// The clock polarity.
        pub clk_pol: ClkPol,
        /// The bit transmission order.
        pub first_bit: FirstBit,
    }

    pub trait SpiSetupInit<
        Spi: SpiMap + SpiCr1,
        SpiInt: IntToken,
        ClkPin: GpioPinMap,
        ClkType: PinTypeToken,
        ClkPull: PinPullToken,
        MisoPin: GpioPinMap,
        MisoType: PinTypeToken,
        MisoPull: PinPullToken,
        MosiPin: GpioPinMap,
        MosiType: PinTypeToken,
        MosiPull: PinPullToken,
        Af: PinAfToken,
        Clk: PClkToken,
    >
    {
        /// Create a new spi setup with sensible defaults.
        fn init(
            spi: SpiPeriph<Spi>,
            spi_int: SpiInt,
            spi_pins: SpiPins<
                ClkPin,
                ClkType,
                ClkPull,
                MisoPin,
                MisoType,
                MisoPull,
                MosiPin,
                MosiType,
                MosiPull,
                Af,
            >,
            clk: ConfiguredClk<Clk>,
            baud_rate: BaudRate,
        ) -> SpiSetup<
            Spi,
            SpiInt,
            ClkPin,
            ClkType,
            ClkPull,
            MisoPin,
            MisoType,
            MisoPull,
            MosiPin,
            MosiType,
            MosiPull,
            Af,
            Clk,
        >;
    }

    macro_rules! spi_setup {
        ($spi:ident, $clk_pin:ident, $miso_pin:ident, $mosi_pin:ident, $pin_af:ident, $pclk:ident) => {
            impl<
                    SpiInt: IntToken,
                    ClkType: PinTypeToken,
                    ClkPull: PinPullToken,
                    MisoType: PinTypeToken,
                    MisoPull: PinPullToken,
                    MosiType: PinTypeToken,
                    MosiPull: PinPullToken,
                >
                SpiSetupInit<
                    drone_stm32_map::periph::spi::$spi,
                    SpiInt,
                    drone_stm32_map::periph::gpio::pin::$clk_pin,
                    ClkType,
                    ClkPull,
                    drone_stm32_map::periph::gpio::pin::$miso_pin,
                    MisoType,
                    MisoPull,
                    drone_stm32_map::periph::gpio::pin::$mosi_pin,
                    MosiType,
                    MosiPull,
                    $pin_af,
                    $pclk,
                >
                for SpiSetup<
                    drone_stm32_map::periph::spi::$spi,
                    SpiInt,
                    drone_stm32_map::periph::gpio::pin::$clk_pin,
                    ClkType,
                    ClkPull,
                    drone_stm32_map::periph::gpio::pin::$miso_pin,
                    MisoType,
                    MisoPull,
                    drone_stm32_map::periph::gpio::pin::$mosi_pin,
                    MosiType,
                    MosiPull,
                    $pin_af,
                    $pclk,
                >
            {
                fn init(
                    spi: SpiPeriph<drone_stm32_map::periph::spi::$spi>,
                    spi_int: SpiInt,
                    spi_pins: SpiPins<
                        drone_stm32_map::periph::gpio::pin::$clk_pin,
                        ClkType,
                        ClkPull,
                        drone_stm32_map::periph::gpio::pin::$miso_pin,
                        MisoType,
                        MisoPull,
                        drone_stm32_map::periph::gpio::pin::$mosi_pin,
                        MosiType,
                        MosiPull,
                        $pin_af,
                    >,
                    clk: ConfiguredClk<$pclk>,
                    baud_rate: BaudRate,
                ) -> SpiSetup<
                    drone_stm32_map::periph::spi::$spi,
                    SpiInt,
                    drone_stm32_map::periph::gpio::pin::$clk_pin,
                    ClkType,
                    ClkPull,
                    drone_stm32_map::periph::gpio::pin::$miso_pin,
                    MisoType,
                    MisoPull,
                    drone_stm32_map::periph::gpio::pin::$mosi_pin,
                    MosiType,
                    MosiPull,
                    $pin_af,
                    $pclk,
                > {
                    Self {
                        spi,
                        spi_int,
                        spi_pins,
                        clk,
                        baud_rate,
                        clk_pol: ClkPol::Low,
                        first_bit: FirstBit::Msb,
                    }
                }
            }
        };
    }
    // The mapping from SPI->AF is in the chips datasheet and not the technical manual.
    // SPI1
    // - CLK:  A5, B3
    // - MISO: A6, B4
    // - MOSI: A7, B5
    spi_setup!(Spi1, GpioA5, GpioA6, GpioA7, PinAf5, PClk2);
    spi_setup!(Spi1, GpioA5, GpioA6, GpioB5, PinAf5, PClk2);
    spi_setup!(Spi1, GpioA5, GpioB4, GpioA7, PinAf5, PClk2);
    spi_setup!(Spi1, GpioA5, GpioB4, GpioB5, PinAf5, PClk2);
    spi_setup!(Spi1, GpioB3, GpioA6, GpioA7, PinAf5, PClk2);
    spi_setup!(Spi1, GpioB3, GpioA6, GpioB5, PinAf5, PClk2);
    spi_setup!(Spi1, GpioB3, GpioB4, GpioA7, PinAf5, PClk2);
    spi_setup!(Spi1, GpioB3, GpioB4, GpioB5, PinAf5, PClk2);

    // SPI1
    // - CLK:  B10, B13, D3, I1
    // - MISO: B14, C2, I2
    // - MOSI: B15, C3, I3
    // spi_setup!(Spi2, PinAf5, PClk2);
    // spi_setup!(Spi3, PinAf5, PClk1);
    // #[cfg(any(
    //     stm32_mcu = "stm32f413",
    //     stm32_mcu = "stm32f427",
    //     stm32_mcu = "stm32f446",
    //     stm32_mcu = "stm32f469",
    // ))]
    // spi_setup!(Spi4, PinAf5, PClk2);
    // #[cfg(any(
    //     stm32_mcu = "stm32f410",
    //     stm32_mcu = "stm32f413",
    //     stm32_mcu = "stm32f427",
    //     stm32_mcu = "stm32f469",
    // ))]
    // spi_setup!(Spi5, PinAf5, PClk2);
    // #[cfg(any(stm32_mcu = "stm32f427", stm32_mcu = "stm32f469",))]
    // spi_setup!(Spi6, PinAf5, PClk2);

    pub enum BaudRate {
        Max(u32),
        Custom(Prescaler),
    }

    #[derive(Copy, Clone, PartialEq)]
    pub enum Prescaler {
        Div2,
        Div4,
        Div8,
        Div16,
        Div32,
        Div64,
        Div128,
        Div256,
    }

    #[derive(Copy, Clone, PartialEq)]
    pub enum ClkPol {
        Low,
        High,
    }

    #[derive(Copy, Clone, PartialEq)]
    pub enum FirstBit {
        Msb,
        Lsb,
    }
}

pub struct SpiDrv<Spi: SpiMap + SpiCr1, SpiInt: IntToken, Clk: PClkToken> {
    spi: SpiDiverged<Spi>,
    spi_int: SpiInt,
    clk: PhantomData<Clk>,
}

impl<Spi: SpiMap + SpiCr1, SpiInt: IntToken, Clk: PClkToken> SpiDrv<Spi, SpiInt, Clk> {
    #[must_use]
    pub fn init<
        ClkPin: GpioPinMap,
        ClkType: PinTypeToken,
        ClkPull: PinPullToken,
        MisoPin: GpioPinMap,
        MisoType: PinTypeToken,
        MisoPull: PinPullToken,
        MosiPin: GpioPinMap,
        MosiType: PinTypeToken,
        MosiPull: PinPullToken,
        Af: PinAfToken,
    >(setup: config::SpiSetup<Spi, SpiInt, ClkPin, ClkType, ClkPull, MisoPin, MisoType, MisoPull, MosiPin, MosiType, MosiPull, Af, Clk>) -> SpiDrv<Spi, SpiInt, Clk> {
        let mut drv = Self {
            spi: setup.spi.into(),
            spi_int: setup.spi_int,
            clk: PhantomData,
        };
        drv.init_spi(setup.clk, setup.baud_rate, setup.clk_pol, setup.first_bit);
        drv
    }

    fn init_master_impl<
        DmaRxCh: DmaChMap,
        DmaRxStCh: DmaStChToken,
        DmaRxInt: IntToken,
        DmaTxCh: DmaChMap,
        DmaTxStCh: DmaStChToken,
        DmaTxInt: IntToken,
    >(
        &self,
        miso_cfg: DmaChCfg<DmaRxCh, DmaRxStCh, DmaRxInt>,
        mosi_cfg: DmaChCfg<DmaTxCh, DmaTxStCh, DmaTxInt>,
    ) -> SpiMasterDrv<Spi, SpiInt, DmaRxCh, DmaRxInt, DmaTxCh, DmaTxInt> {
        let DmaChCfg {
            dma_ch: dma_rx,
            dma_int: dma_rx_int,
            dma_pl: dma_rx_pl,
            ..
        } = miso_cfg;
        let DmaChCfg {
            dma_ch: dma_tx,
            dma_int: dma_tx_int,
            dma_pl: dma_tx_pl,
            ..
        } = mosi_cfg;
        let mut master = SpiMasterDrv {
            spi: &self.spi,
            spi_int: &self.spi_int,
            dma_rx: dma_rx.into(),
            dma_rx_int,
            dma_tx: dma_tx.into(),
            dma_tx_int,
        };

        master.init_dma_rx(DmaRxStCh::num(), dma_rx_pl);
        master.init_dma_tx(DmaTxStCh::num(), dma_tx_pl);

        master
    }

    fn init_spi(
        &mut self,
        clk: ConfiguredClk<Clk>,
        baud_rate: BaudRate,
        clk_pol: ClkPol,
        first_bit: FirstBit,
    ) {
        use self::config::*;

        // Enable spi clock.
        self.spi.rcc_busenr_spien.set_bit();

        // Configure spi.
        self.spi.spi_cr1.store_reg(|r, v| {
            // Do not enable spi before it is fully configured.

            // Use software slave management, i.e. the app controls slave selection.
            r.ssm().set(v);

            if first_bit == FirstBit::Lsb {
                r.lsbfirst().set(v);
            }

            // Baud rate control.
            r.br().write(v, spi_br(clk, baud_rate));

            // Master configuration.
            r.mstr().set(v);

            // Clock polarity.
            if clk_pol == ClkPol::High {
                r.cpol().set(v);
            }

            // Clock phase.
            // TODO: Expose configuration option?
            r.cpha().clear(v);
        });

        self.spi.spi_cr2.store_reg(|r, v| {
            // Enable error interrupt
            r.errie().set(v);
        });

        self.spi.spi_cr1.modify_reg(|r, v| {
            // Enable spi after being fully configured.
            r.spe().set(v);
        });

        // Attach spi error handler
        let sr = self.spi.spi_sr;
        self.spi_int.add_fn(move || {
            let val = sr.load_val();
            handle_spi_err::<Spi>(&val, sr);
            fib::Yielded::<(), !>(())
        });
    }
}

pub trait SpiDrvInit<
    Spi: SpiMap,
    SpiInt: IntToken,
    DmaRxCh: DmaChMap,
    DmaRxStCh: DmaStChToken,
    DmaTxCh: DmaChMap,
    DmaTxStCh: DmaStChToken,
>
{
    fn init_master<DmaRxInt: IntToken, DmaTxInt: IntToken>(
        &self,
        miso_cfg: DmaChCfg<DmaRxCh, DmaRxStCh, DmaRxInt>,
        mosi_cfg: DmaChCfg<DmaTxCh, DmaTxStCh, DmaTxInt>,
    ) -> SpiMasterDrv<Spi, SpiInt, DmaRxCh, DmaRxInt, DmaTxCh, DmaTxInt>;
}

macro_rules! master_drv_init {
    ($spi:ident, $miso_ch:ident, $miso_stch:ident, $mosi_ch:ident, $mosi_stch:ident) => {
        impl<SpiInt: IntToken, Clk: PClkToken>
            SpiDrvInit<
                drone_stm32_map::periph::spi::$spi,
                SpiInt,
                drone_stm32_map::periph::dma::ch::$miso_ch,
                $miso_stch,
                drone_stm32_map::periph::dma::ch::$mosi_ch,
                $mosi_stch,
            > for SpiDrv<drone_stm32_map::periph::spi::$spi, SpiInt, Clk>
        {
            fn init_master<DmaRxInt: IntToken, DmaTxInt: IntToken>(
                &self,
                miso_cfg: DmaChCfg<
                    drone_stm32_map::periph::dma::ch::$miso_ch,
                    $miso_stch,
                    DmaRxInt,
                >,
                mosi_cfg: DmaChCfg<
                    drone_stm32_map::periph::dma::ch::$mosi_ch,
                    $mosi_stch,
                    DmaTxInt,
                >,
            ) -> SpiMasterDrv<
                drone_stm32_map::periph::spi::$spi,
                SpiInt,
                drone_stm32_map::periph::dma::ch::$miso_ch,
                DmaRxInt,
                drone_stm32_map::periph::dma::ch::$mosi_ch,
                DmaTxInt,
            > {
                self.init_master_impl(miso_cfg, mosi_cfg)
            }
        }
    };
}

// This configuration reflect the dma mappings in table 42 and 43 in PM0090.
master_drv_init!(Spi1, Dma2Ch0, DmaStCh3, Dma2Ch3, DmaStCh3);
master_drv_init!(Spi1, Dma2Ch0, DmaStCh3, Dma2Ch5, DmaStCh3);
master_drv_init!(Spi1, Dma2Ch2, DmaStCh3, Dma2Ch3, DmaStCh3);
master_drv_init!(Spi1, Dma2Ch2, DmaStCh3, Dma2Ch5, DmaStCh3);
master_drv_init!(Spi2, Dma1Ch3, DmaStCh0, Dma1Ch4, DmaStCh0);
master_drv_init!(Spi3, Dma1Ch0, DmaStCh0, Dma1Ch5, DmaStCh0);
master_drv_init!(Spi3, Dma1Ch0, DmaStCh0, Dma1Ch7, DmaStCh0);
master_drv_init!(Spi3, Dma1Ch2, DmaStCh0, Dma1Ch5, DmaStCh0);
master_drv_init!(Spi3, Dma1Ch2, DmaStCh0, Dma1Ch7, DmaStCh0);
#[cfg(any(
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
master_drv_init!(Spi4, Dma2Ch0, DmaStCh4, Dma2Ch1, DmaStCh4);
#[cfg(any(
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
master_drv_init!(Spi4, Dma2Ch0, DmaStCh4, Dma2Ch4, DmaStCh5);
#[cfg(any(
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
master_drv_init!(Spi4, Dma2Ch3, DmaStCh5, Dma2Ch1, DmaStCh4);
#[cfg(any(
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
master_drv_init!(Spi4, Dma2Ch3, DmaStCh5, Dma2Ch4, DmaStCh5);
#[cfg(any(
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
master_drv_init!(Spi5, Dma2Ch3, DmaStCh2, Dma2Ch4, DmaStCh2);
#[cfg(any(
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
master_drv_init!(Spi5, Dma2Ch3, DmaStCh2, Dma2Ch6, DmaStCh7);
#[cfg(any(
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
master_drv_init!(Spi5, Dma2Ch5, DmaStCh7, Dma2Ch4, DmaStCh2);
#[cfg(any(
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
master_drv_init!(Spi5, Dma2Ch5, DmaStCh7, Dma2Ch6, DmaStCh7);
#[cfg(any(stm32_mcu = "stm32f427", stm32_mcu = "stm32f469",))]
master_drv_init!(Spi6, Dma2Ch6, DmaStCh1, Dma2Ch5, DmaStCh1);

fn spi_br<Clk: PClkToken>(clk: ConfiguredClk<Clk>, baud_rate: BaudRate) -> u32 {
    use config::*;
    let f_pclk = clk.freq();
    let presc = match baud_rate {
        BaudRate::Max(baud_rate) => match f_pclk / baud_rate {
            0 => unreachable!(),
            1..=2 => Prescaler::Div2,
            3..=4 => Prescaler::Div4,
            5..=8 => Prescaler::Div8,
            9..=16 => Prescaler::Div16,
            17..=32 => Prescaler::Div32,
            33..=64 => Prescaler::Div64,
            65..=128 => Prescaler::Div128,
            _ => Prescaler::Div256,
        },
        BaudRate::Custom(prescaler) => prescaler,
    };

    match presc {
        Prescaler::Div2 => 0b000,
        Prescaler::Div4 => 0b001,
        Prescaler::Div8 => 0b010,
        Prescaler::Div16 => 0b011,
        Prescaler::Div32 => 0b100,
        Prescaler::Div64 => 0b101,
        Prescaler::Div128 => 0b110,
        Prescaler::Div256 => 0b111,
    }
}

fn handle_spi_err<Spi: SpiMap>(val: &Spi::SpiSrVal, sr: Spi::CSpiSr) {
    if sr.fre().read(&val) {
        panic!("Frame format error");
    }
    if sr.ovr().read(&val) {
        panic!("Overrun error");
    }
    if sr.modf().read(&val) {
        panic!("Mode fault");
    }
    if sr.crcerr().read(&val) {
        panic!("CRC error");
    }
    if sr.udr().read(&val) {
        panic!("Underrun error");
    }
}

pub(crate) fn handle_dma_err<T: DmaChMap>(
    val: &T::DmaIsrVal,
    dma_isr_dmeif: T::CDmaIsrDmeif,
    dma_isr_feif: T::CDmaIsrFeif,
    dma_isr_teif: T::CDmaIsrTeif,
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
