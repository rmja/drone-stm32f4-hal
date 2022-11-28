use alloc::sync::Arc;
use core::marker::PhantomData;
use drone_cortexm::reg::prelude::*;
use drone_stm32_map::periph::gpio::{
    head::GpioHeadMap,
    pin::{GpioPinMap, GpioPinPeriph},
};

pub trait PinModeOrDontCare: Send + Sync + 'static {}
pub trait PinModeMap: PinModeOrDontCare {}

pub trait PinTypeOrDontCare: Send + Sync + 'static {}
pub trait PinTypeMap: PinTypeOrDontCare {}

pub trait PinPullMap: Send + Sync + 'static {}

pub trait PinAf: Send + Sync + 'static {
    const NUM: u32;
}

/// Pin configuration.
pub struct GpioPin<
    Pin: GpioPinMap,
    Mode: PinModeOrDontCare,
    Type: PinTypeOrDontCare,
    Pull: PinPullMap,
> {
    pub(crate) pin: Arc<GpioPinPeriph<Pin>>,
    mode: PhantomData<Mode>,
    type_: PhantomData<Type>,
    pull: PhantomData<Pull>,
}

impl<Pin: GpioPinMap, Mode: PinModeOrDontCare, Type: PinTypeOrDontCare, Pull: PinPullMap>
    From<Arc<GpioPinPeriph<Pin>>> for GpioPin<Pin, Mode, Type, Pull>
{
    fn from(pin: Arc<GpioPinPeriph<Pin>>) -> Self {
        Self {
            pin,
            mode: PhantomData,
            type_: PhantomData,
            pull: PhantomData,
        }
    }
}

/// Generic dont-care mode for undefined state.
pub struct DontCare;
impl PinModeOrDontCare for DontCare {}
impl PinTypeOrDontCare for DontCare {}

/// General purpose input mode (MODER=0b00).
pub struct InputMode;
impl PinModeMap for InputMode {}
impl PinModeOrDontCare for InputMode {}

/// General purpose output mode  (MODER=0b01).
pub struct OutputMode;
impl PinModeMap for OutputMode {}
impl PinModeOrDontCare for OutputMode {}

/// Alternate function mode  (MODER=0b10).
pub struct AlternateMode<Af: PinAf> {
    af: PhantomData<Af>,
}
impl<Af: PinAf> PinModeMap for AlternateMode<Af> {}
impl<Af: PinAf> PinModeOrDontCare for AlternateMode<Af> {}

// TODO: Analog mode

/// Push/pull type (OTYPER=0).
/// This is only applicabale for OutputMode and AlternateMode.
pub struct PushPullType;
impl PinTypeMap for PushPullType {}
impl PinTypeOrDontCare for PushPullType {}

/// Output open-drain type (OTYPER=1).
/// This is only applicabale for OutputMode and AlternateMode.
pub struct OpenDrainType;
impl PinTypeMap for OpenDrainType {}
impl PinTypeOrDontCare for OpenDrainType {}

/// No pull-up nor pull-down. For inputs this means floating.
pub struct NoPull;
impl PinPullMap for NoPull {}

/// Pull up.
pub struct PullUp;
impl PinPullMap for PullUp {}

/// Pull down.
pub struct PullDown;
impl PinPullMap for PullDown {}

pub struct PinAf0;
pub struct PinAf1;
pub struct PinAf2;
pub struct PinAf3;
pub struct PinAf4;
pub struct PinAf5;
pub struct PinAf6;
pub struct PinAf7;
pub struct PinAf8;
pub struct PinAf9;
pub struct PinAf10;
pub struct PinAf11;
pub struct PinAf12;
pub struct PinAf13;
pub struct PinAf14;
pub struct PinAf15;

macro_rules! af_token {
    ($af:ident, $num:expr) => {
        impl PinAf for $af {
            const NUM: u32 = $num;
        }
    };
}

af_token!(PinAf0, 0);
af_token!(PinAf1, 1);
af_token!(PinAf2, 2);
af_token!(PinAf3, 3);
af_token!(PinAf4, 4);
af_token!(PinAf5, 5);
af_token!(PinAf6, 6);
af_token!(PinAf7, 7);
af_token!(PinAf8, 8);
af_token!(PinAf9, 9);
af_token!(PinAf10, 10);
af_token!(PinAf11, 11);
af_token!(PinAf12, 12);
af_token!(PinAf13, 13);
af_token!(PinAf14, 14);
af_token!(PinAf15, 15);
af_token!(DontCare, 0);

/// Gpio pin speed.
pub enum GpioPinSpeed {
    LowSpeed,
    MediumSpeed,
    HighSpeed,
    VeryHighSpeed,
}

impl<Pin: GpioPinMap> GpioPin<Pin, DontCare, DontCare, NoPull> {
    /// Set pin into general purpose input mode.
    pub fn into_input(self) -> GpioPin<Pin, InputMode, DontCare, NoPull> {
        self.pin.gpio_moder_moder.write_bits(0b00);
        self.pin.into()
    }

    /// Set pin into general purpose output mode.
    pub fn into_output(self) -> GpioPin<Pin, OutputMode, DontCare, NoPull> {
        self.pin.gpio_moder_moder.write_bits(0b01);
        self.pin.into()
    }

    /// Set the pin into alternate function mode.
    pub fn into_alternate<Af: PinAf>(self) -> GpioPin<Pin, AlternateMode<Af>, DontCare, NoPull> {
        self.pin.gpio_afr_afr.write_bits(Af::NUM);
        self.pin.gpio_moder_moder.write_bits(0b10);
        self.pin.into()
    }
}

pub trait TypeModes: PinModeMap {}
impl TypeModes for InputMode {}
impl TypeModes for OutputMode {}
impl<Af: PinAf> TypeModes for AlternateMode<Af> {}

impl<Pin: GpioPinMap, Mode: TypeModes> GpioPin<Pin, Mode, DontCare, NoPull> {
    /// Let pin type be push/pull.
    pub fn into_pushpull(self) -> GpioPin<Pin, Mode, PushPullType, NoPull> {
        self.pin.gpio_otyper_ot.clear_bit();
        self.pin.gpio_pupdr_pupdr.write_bits(0b00); // No pull-up nor pull-down.
        self.pin.into()
    }

    /// Let pin type be open-drain.
    pub fn into_opendrain(self) -> GpioPin<Pin, Mode, OpenDrainType, NoPull> {
        self.pin.gpio_otyper_ot.set_bit();
        self.pin.into()
    }
}

pub trait PullModes: PinModeMap {}
impl PullModes for InputMode {}
impl PullModes for OutputMode {}
impl<Af: PinAf> PullModes for AlternateMode<Af> {}

impl<Pin: GpioPinMap, Mode: PullModes> GpioPin<Pin, Mode, PushPullType, NoPull> {
    /// No pull-up nor pull-down (this is the default).
    pub fn into_nopull(self) -> GpioPin<Pin, Mode, PushPullType, NoPull> {
        self.pin.gpio_pupdr_pupdr.write_bits(0b00);
        self.pin.into()
    }

    /// Let pin be pulled-up.
    pub fn into_pullup(self) -> GpioPin<Pin, Mode, PushPullType, PullUp> {
        self.pin.gpio_pupdr_pupdr.write_bits(0b01);
        self.pin.into()
    }

    /// Let pin be pulled-down.
    pub fn into_pulldown(self) -> GpioPin<Pin, Mode, PushPullType, PullDown> {
        self.pin.gpio_pupdr_pupdr.write_bits(0b10);
        self.pin.into()
    }
}

pub trait WithSpeedModes: PinModeMap {}
impl WithSpeedModes for OutputMode {}
impl<Af: PinAf> WithSpeedModes for AlternateMode<Af> {}

impl<Pin: GpioPinMap, Mode: WithSpeedModes, Type: PinTypeOrDontCare, Pull: PinPullMap>
    GpioPin<Pin, Mode, Type, Pull>
{
    /// Set pin speed.
    pub fn with_speed(self, speed: GpioPinSpeed) -> Self {
        self.pin.gpio_ospeedr_ospeedr.write_bits(match speed {
            GpioPinSpeed::LowSpeed => 0,
            GpioPinSpeed::MediumSpeed => 1,
            GpioPinSpeed::HighSpeed => 2,
            GpioPinSpeed::VeryHighSpeed => 3,
        });
        self
    }
}

pub trait PinGetMode: PinModeMap {}
impl PinGetMode for InputMode {}
impl PinGetMode for OutputMode {}
impl<Af: PinAf> PinGetMode for AlternateMode<Af> {}

impl<Pin: GpioPinMap, Mode: PinGetMode, Type: PinTypeMap, Pull: PinPullMap>
    GpioPin<Pin, Mode, Type, Pull>
{
    /// Get the current pin state.
    pub fn get(&self) -> bool {
        self.pin.gpio_idr_idr.read_bit()
    }
}

impl<Pin: GpioPinMap, Type: PinTypeMap, Pull: PinPullMap> GpioPin<Pin, OutputMode, Type, Pull> {
    /// Set output pin high.
    #[inline]
    pub fn set(&self) {
        // Set output pin to high by writing BS (bit set) to the bit set/reset register.
        self.pin.gpio_bsrr_bs.set_bit();
    }

    /// Set output pin low.
    #[inline]
    pub fn clear(&self) {
        // Clear output pin to low by writing BR (bit reset) to the bit set/reset register.
        self.pin.gpio_bsrr_br.set_bit();
    }
}

impl<Pin: GpioPinMap, Mode: PinModeMap, Type: PinTypeMap, Pull: PinPullMap>
    GpioPin<Pin, Mode, Type, Pull>
{
    /// Clone the pin
    ///
    /// # Safety
    /// The function is unsafe as there are no guarantees that the two configuration can co-exist.
    pub unsafe fn clone(&self) -> Self {
        Self {
            pin: self.pin.clone(),
            mode: self.mode,
            type_: self.type_,
            pull: self.pull,
        }
    }
}

pub trait NewPin<Head: GpioHeadMap, Pin: GpioPinMap> {
    /// Create a new pin configuration from a pin peripheral.
    fn pin(&self, pin: GpioPinPeriph<Pin>) -> GpioPin<Pin, DontCare, DontCare, NoPull>;
}

#[macro_export]
macro_rules! pin_init {
    ($($head:ident, $pin:ident;)+) => {
        $(
            impl
                crate::pin::NewPin<
                    $head,
                    $pin,
                > for crate::head::GpioHead<$head>
            {
                fn pin(
                    &self,
                    pin: ::drone_stm32_map::periph::gpio::pin::GpioPinPeriph<
                        $pin,
                    >,
                ) -> crate::pin::GpioPin<
                    $pin,
                    crate::pin::DontCare,
                    crate::pin::DontCare,
                    crate::NoPull,
                > {
                    crate::pin::GpioPin::from(alloc::sync::Arc::new(pin))
                }
            }
        )+
    };
}
