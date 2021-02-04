// For signal names, see Table 296 in PM0090.

use core::marker::PhantomData;

/// Defined marker type.
pub struct D;

/// Undefined marker type.
pub struct U;

pub trait Opt {
    const NUM: u32;
}

impl Opt for D {
    const NUM: u32 = 1;
}

impl Opt for U {
    const NUM: u32 = 0;
}

// See table 21 in stm32f429 datasheet
#[macro_export]
macro_rules! sdram_pin_periph {
    (gpio_f, a0, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_f0!($reg) };
    (gpio_f, a1, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_f1!($reg) };
    (gpio_f, a2, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_f2!($reg) };
    (gpio_f, a3, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_f3!($reg) };
    (gpio_f, a4, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_f4!($reg) };
    (gpio_f, a5, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_f5!($reg) };
    (gpio_f, a6, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_f12!($reg) };
    (gpio_f, a7, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_f13!($reg) };
    (gpio_f, a8, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_f14!($reg) };
    (gpio_f, a9, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_f15!($reg) };
    (gpio_g, a10, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_g0!($reg) };
    (gpio_g, a11, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_g1!($reg) };
    (gpio_g, a12, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_g2!($reg) };

    (gpio_g, ba0, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_g4!($reg) };
    (gpio_g, ba1, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_g5!($reg) };

    (gpio_d, d0, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_d14!($reg) };
    (gpio_d, d1, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_d15!($reg) };
    (gpio_d, d2, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_d0!($reg) };
    (gpio_d, d3, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_d1!($reg) };
    (gpio_e, d4, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_e7!($reg) };
    (gpio_e, d5, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_e8!($reg) };
    (gpio_e, d6, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_e9!($reg) };
    (gpio_e, d7, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_e10!($reg) };
    (gpio_e, d8, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_e11!($reg) };
    (gpio_e, d9, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_e12!($reg) };
    (gpio_e, d10, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_e13!($reg) };
    (gpio_e, d11, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_e14!($reg) };
    (gpio_e, d12, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_e15!($reg) };
    (gpio_d, d13, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_d8!($reg) };
    (gpio_d, d14, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_d9!($reg) };
    (gpio_d, d15, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_d10!($reg) };
    (gpio_h, d16, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_h8!($reg) };
    (gpio_h, d17, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_h9!($reg) };
    (gpio_h, d18, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_h10!($reg) };
    (gpio_h, d19, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_h11!($reg) };
    (gpio_h, d20, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_h12!($reg) };
    (gpio_h, d21, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_h13!($reg) };
    (gpio_h, d22, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_h14!($reg) };
    (gpio_h, d23, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_h15!($reg) };
    (gpio_i, d24, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_i0!($reg) };
    (gpio_i, d25, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_i1!($reg) };
    (gpio_i, d26, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_i2!($reg) };
    (gpio_i, d27, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_i3!($reg) };
    (gpio_i, d28, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_i6!($reg) };
    (gpio_i, d29, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_i7!($reg) };
    (gpio_i, d30, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_i9!($reg) };
    (gpio_i, d31, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_i10!($reg) };

    (gpio_e, nbl0, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_e0!($reg) };
    (gpio_e, nbl1, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_e1!($reg) };
    (gpio_i, nbl2, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_i4!($reg) };
    (gpio_i, nbl3, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_i5!($reg) };

    (gpio_g, sdclk, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_g8!($reg) };
    (gpio_c, sdnwe, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_c0!($reg) };
    (gpio_f, sdnras, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_f11!($reg) };
    (gpio_g, sdncas, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_g15!($reg) };
    (gpio_h, sdcke0, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_h2!($reg) };
    (gpio_h, sdne0, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_h3!($reg) };
    (gpio_h, sdne1, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_h6!($reg) };
    (gpio_h, sdcke1, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_h7!($reg) };
    (gpio_h, sdnwe, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_h5!($reg) };
    (gpio_c, sdne0, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_c2!($reg) };
    (gpio_c, sdcke0, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_c3!($reg) };
    (gpio_b, sdcke1, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_b5!($reg) };
    (gpio_b, sdne1, $reg:ident) => { ::drone_stm32_map::periph::gpio::periph_gpio_b6!($reg) };
}

#[macro_export]
macro_rules! sdram_pins {
    (helper, $pins:ident, $reg:ident, $gpio:ident => $pin_name:ident;) => {
        {
            let periph = ::drone_stm32f4_hal::fmc::sdram_pin_periph!($gpio, $pin_name, $reg);
            let pin = ::drone_stm32f4_hal::gpio::NewPin::pin(&$gpio, periph);
            $pins.$pin_name(pin.into_alternate().into_pushpull().with_speed(::drone_stm32f4_hal::gpio::GpioPinSpeed::HighSpeed))
        }
    };
    (helper, $pins:ident, $reg:ident, $gpio:ident => $($pin_name:ident),+;) => {
        {
            let pins = $pins;
            $(
                let periph = ::drone_stm32f4_hal::fmc::sdram_pin_periph!($gpio, $pin_name, $reg);
                let pin = ::drone_stm32f4_hal::gpio::NewPin::pin(&$gpio, periph);
                let pins = pins.$pin_name(pin.into_alternate().into_pushpull().with_speed(::drone_stm32f4_hal::gpio::GpioPinSpeed::HighSpeed));
            )+
            pins
        }
    };
    (helper, $pins:ident, $reg:ident, $gpio:ident => $($pin_name:ident),+;$($tail:tt)*) => {
        {
            let pins = sdram_pins!(helper, $pins, $reg, $gpio => $($pin_name),+;);
            sdram_pins!(helper, pins, $reg, $($tail)*)
        }
    };
    ($type:ident, $reg:ident, $($mappings:tt)*) => {
        {
            let pins = $type::default();
            sdram_pins!(helper, pins, $reg, $($mappings)*)
        }
    };
}

pub struct FmcSdRamPins<Sdclk, Sdcke0, Sdcke1, Sdne0, Sdne1, Nras, Ncas, Sdnwe> {
    sdclk: PhantomData<Sdclk>,
    sdcke0: PhantomData<Sdcke0>,
    sdcke1: PhantomData<Sdcke1>,
    sdne0: PhantomData<Sdne0>,
    sdne1: PhantomData<Sdne1>,
    nras: PhantomData<Nras>,
    ncas: PhantomData<Ncas>,
    sdnwe: PhantomData<Sdnwe>,
}

impl<Sdclk, Sdcke0, Sdcke1, Sdne0, Sdne1, Nras, Ncas, Sdnwe>
    FmcSdRamPins<Sdclk, Sdcke0, Sdcke1, Sdne0, Sdne1, Nras, Ncas, Sdnwe>
{
    pub fn new() -> Self {
        Self {
            sdclk: PhantomData,
            sdcke0: PhantomData,
            sdcke1: PhantomData,
            sdne0: PhantomData,
            sdne1: PhantomData,
            nras: PhantomData,
            ncas: PhantomData,
            sdnwe: PhantomData,
        }
    }
}

impl Default for FmcSdRamPins<U, U, U, U, U, U, U, U> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct FmcSdRamAddressPins<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12> {
    a0: PhantomData<A0>,
    a1: PhantomData<A1>,
    a2: PhantomData<A2>,
    a3: PhantomData<A3>,
    a4: PhantomData<A4>,
    a5: PhantomData<A5>,
    a6: PhantomData<A6>,
    a7: PhantomData<A7>,
    a8: PhantomData<A8>,
    a9: PhantomData<A9>,
    a10: PhantomData<A10>,
    a11: PhantomData<A11>,
    a12: PhantomData<A12>,
}

impl<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12>
    FmcSdRamAddressPins<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12>
{
    pub fn new() -> Self {
        Self {
            a0: PhantomData,
            a1: PhantomData,
            a2: PhantomData,
            a3: PhantomData,
            a4: PhantomData,
            a5: PhantomData,
            a6: PhantomData,
            a7: PhantomData,
            a8: PhantomData,
            a9: PhantomData,
            a10: PhantomData,
            a11: PhantomData,
            a12: PhantomData,
        }
    }
}

impl Default for FmcSdRamAddressPins<U, U, U, U, U, U, U, U, U, U, U, U, U> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct FmcSdRamDataPins<
    D0,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
    D10,
    D11,
    D12,
    D13,
    D14,
    D15,
    D16,
    D17,
    D18,
    D19,
    D20,
    D21,
    D22,
    D23,
    D24,
    D25,
    D26,
    D27,
    D28,
    D29,
    D30,
    D31,
> {
    d0: PhantomData<D0>,
    d1: PhantomData<D1>,
    d2: PhantomData<D2>,
    d3: PhantomData<D3>,
    d4: PhantomData<D4>,
    d5: PhantomData<D5>,
    d6: PhantomData<D6>,
    d7: PhantomData<D7>,
    d8: PhantomData<D8>,
    d9: PhantomData<D9>,
    d10: PhantomData<D10>,
    d11: PhantomData<D11>,
    d12: PhantomData<D12>,
    d13: PhantomData<D13>,
    d14: PhantomData<D14>,
    d15: PhantomData<D15>,
    d16: PhantomData<D16>,
    d17: PhantomData<D17>,
    d18: PhantomData<D18>,
    d19: PhantomData<D19>,
    d20: PhantomData<D20>,
    d21: PhantomData<D21>,
    d22: PhantomData<D22>,
    d23: PhantomData<D23>,
    d24: PhantomData<D24>,
    d25: PhantomData<D25>,
    d26: PhantomData<D26>,
    d27: PhantomData<D27>,
    d28: PhantomData<D28>,
    d29: PhantomData<D29>,
    d30: PhantomData<D30>,
    d31: PhantomData<D31>,
}

impl<
        D0,
        D1,
        D2,
        D3,
        D4,
        D5,
        D6,
        D7,
        D8,
        D9,
        D10,
        D11,
        D12,
        D13,
        D14,
        D15,
        D16,
        D17,
        D18,
        D19,
        D20,
        D21,
        D22,
        D23,
        D24,
        D25,
        D26,
        D27,
        D28,
        D29,
        D30,
        D31,
    >
    FmcSdRamDataPins<
        D0,
        D1,
        D2,
        D3,
        D4,
        D5,
        D6,
        D7,
        D8,
        D9,
        D10,
        D11,
        D12,
        D13,
        D14,
        D15,
        D16,
        D17,
        D18,
        D19,
        D20,
        D21,
        D22,
        D23,
        D24,
        D25,
        D26,
        D27,
        D28,
        D29,
        D30,
        D31,
    >
{
    pub fn new() -> Self {
        Self {
            d0: PhantomData,
            d1: PhantomData,
            d2: PhantomData,
            d3: PhantomData,
            d4: PhantomData,
            d5: PhantomData,
            d6: PhantomData,
            d7: PhantomData,
            d8: PhantomData,
            d9: PhantomData,
            d10: PhantomData,
            d11: PhantomData,
            d12: PhantomData,
            d13: PhantomData,
            d14: PhantomData,
            d15: PhantomData,
            d16: PhantomData,
            d17: PhantomData,
            d18: PhantomData,
            d19: PhantomData,
            d20: PhantomData,
            d21: PhantomData,
            d22: PhantomData,
            d23: PhantomData,
            d24: PhantomData,
            d25: PhantomData,
            d26: PhantomData,
            d27: PhantomData,
            d28: PhantomData,
            d29: PhantomData,
            d30: PhantomData,
            d31: PhantomData,
        }
    }
}

impl Default
    for FmcSdRamDataPins<
        U,
        U,
        U,
        U,
        U,
        U,
        U,
        U,
        U,
        U,
        U,
        U,
        U,
        U,
        U,
        U,
        U,
        U,
        U,
        U,
        U,
        U,
        U,
        U,
        U,
        U,
        U,
        U,
        U,
        U,
        U,
        U,
    >
{
    fn default() -> Self {
        Self::new()
    }
}

pub struct FmcSdRamBankPins<BA0, BA1> {
    ba0: PhantomData<BA0>,
    ba1: PhantomData<BA1>,
}

impl<BA0, BA1> FmcSdRamBankPins<BA0, BA1> {
    pub fn new() -> Self {
        Self {
            ba0: PhantomData,
            ba1: PhantomData,
        }
    }
}

impl Default for FmcSdRamBankPins<U, U> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct FmcSdRamByteMaskPins<NBL0, NBL1, NBL2, NBL3> {
    nbl0: PhantomData<NBL0>,
    nbl1: PhantomData<NBL1>,
    nbl2: PhantomData<NBL2>,
    nbl3: PhantomData<NBL3>,
}

impl<NBL0, NBL1, NBL2, NBL3> FmcSdRamByteMaskPins<NBL0, NBL1, NBL2, NBL3> {
    pub fn new() -> Self {
        Self {
            nbl0: PhantomData,
            nbl1: PhantomData,
            nbl2: PhantomData,
            nbl3: PhantomData,
        }
    }
}

impl Default for FmcSdRamByteMaskPins<U, U, U, U> {
    fn default() -> Self {
        Self::new()
    }
}

pub mod traits {
    use super::*;
    use drone_stm32f4_gpio_drv::pin_ext;

    pin_ext!(SdclkPinExt<..., Sdclk, Sdcke0, Sdcke1, Sdne0, Sdne1, Sdnras, Sdncas, Sdnwe>.sdclk -> FmcSdRamPins<D, Sdcke0, Sdcke1, Sdne0, Sdne1, Sdnras, Sdncas, Sdnwe>);
    pin_ext!(Sdcke0PinExt<..., Sdclk, Sdcke0, Sdcke1, Sdne0, Sdne1, Sdnras, Sdncas, Sdnwe>.sdcke0 -> FmcSdRamPins<Sdclk, D, Sdcke1, Sdne0, Sdne1, Sdnras, Sdncas, Sdnwe>);
    pin_ext!(Sdcke1PinExt<..., Sdclk, Sdcke0, Sdcke1, Sdne0, Sdne1, Sdnras, Sdncas, Sdnwe>.sdcke1 -> FmcSdRamPins<Sdclk, Sdcke0, D, Sdne0, Sdne1, Sdnras, Sdncas, Sdnwe>);
    pin_ext!(Sdne0PinExt<..., Sdclk, Sdcke0, Sdcke1, Sdne0, Sdne1, Sdnras, Sdncas, Sdnwe>.sdne0 -> FmcSdRamPins<Sdclk, Sdcke0, Sdcke1, D, Sdne1, Sdnras, Sdncas, Sdnwe>);
    pin_ext!(Sdne1PinExt<..., Sdclk, Sdcke0, Sdcke1, Sdne0, Sdne1, Sdnras, Sdncas, Sdnwe>.sdne1 -> FmcSdRamPins<Sdclk, Sdcke0, Sdcke1, Sdne0, D, Sdnras, Sdncas, Sdnwe>);
    pin_ext!(SdnrasPinExt<..., Sdclk, Sdcke0, Sdcke1, Sdne0, Sdne1, Sdnras, Sdncas, Sdnwe>.sdnras -> FmcSdRamPins<Sdclk, Sdcke0, Sdcke1, Sdne0, Sdne1, D, Sdncas, Sdnwe>);
    pin_ext!(SdncasPinExt<..., Sdclk, Sdcke0, Sdcke1, Sdne0, Sdne1, Sdnras, Sdncas, Sdnwe>.sdncas -> FmcSdRamPins<Sdclk, Sdcke0, Sdcke1, Sdne0, Sdne1, Sdnras, D, Sdnwe>);
    pin_ext!(SdnwePinExt<..., Sdclk, Sdcke0, Sdcke1, Sdne0, Sdne1, Sdnras, Sdncas, Sdnwe>.sdnwe -> FmcSdRamPins<Sdclk, Sdcke0, Sdcke1, Sdne0, Sdne1, Sdnras, Sdncas, D>);

    pin_ext!( A0PinExt.a0  -> FmcSdRamAddressPins<D, U, U, U, U, U, U, U, U, U, U, U, U>);
    pin_ext!( A1PinExt.a1  -> FmcSdRamAddressPins<D, D, U, U, U, U, U, U, U, U, U, U, U>);
    pin_ext!( A2PinExt.a2  -> FmcSdRamAddressPins<D, D, D, U, U, U, U, U, U, U, U, U, U>);
    pin_ext!( A3PinExt.a3  -> FmcSdRamAddressPins<D, D, D, D, U, U, U, U, U, U, U, U, U>);
    pin_ext!( A4PinExt.a4  -> FmcSdRamAddressPins<D, D, D, D, D, U, U, U, U, U, U, U, U>);
    pin_ext!( A5PinExt.a5  -> FmcSdRamAddressPins<D, D, D, D, D, D, U, U, U, U, U, U, U>);
    pin_ext!( A6PinExt.a6  -> FmcSdRamAddressPins<D, D, D, D, D, D, D, U, U, U, U, U, U>);
    pin_ext!( A7PinExt.a7  -> FmcSdRamAddressPins<D, D, D, D, D, D, D, D, U, U, U, U, U>);
    pin_ext!( A8PinExt.a8  -> FmcSdRamAddressPins<D, D, D, D, D, D, D, D, D, U, U, U, U>);
    pin_ext!( A9PinExt.a9  -> FmcSdRamAddressPins<D, D, D, D, D, D, D, D, D, D, U, U, U>);
    pin_ext!(A10PinExt.a10 -> FmcSdRamAddressPins<D, D, D, D, D, D, D, D, D, D, D, U, U>);
    pin_ext!(A11PinExt.a11 -> FmcSdRamAddressPins<D, D, D, D, D, D, D, D, D, D, D, D, U>);
    pin_ext!(A12PinExt.a12 -> FmcSdRamAddressPins<D, D, D, D, D, D, D, D, D, D, D, D, D>);

    pin_ext!( D0PinExt.d0  -> FmcSdRamDataPins<D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U>);
    pin_ext!( D1PinExt.d1  -> FmcSdRamDataPins<D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U>);
    pin_ext!( D2PinExt.d2  -> FmcSdRamDataPins<D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U>);
    pin_ext!( D3PinExt.d3  -> FmcSdRamDataPins<D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U>);
    pin_ext!( D4PinExt.d4  -> FmcSdRamDataPins<D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U>);
    pin_ext!( D5PinExt.d5  -> FmcSdRamDataPins<D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U>);
    pin_ext!( D6PinExt.d6  -> FmcSdRamDataPins<D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U>);
    pin_ext!( D7PinExt.d7  -> FmcSdRamDataPins<D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U>);
    pin_ext!( D8PinExt.d8  -> FmcSdRamDataPins<D, D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U>);
    pin_ext!( D9PinExt.d9  -> FmcSdRamDataPins<D, D, D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U>);
    pin_ext!(D10PinExt.d10 -> FmcSdRamDataPins<D, D, D, D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U>);
    pin_ext!(D11PinExt.d11 -> FmcSdRamDataPins<D, D, D, D, D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U>);
    pin_ext!(D12PinExt.d12 -> FmcSdRamDataPins<D, D, D, D, D, D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U>);
    pin_ext!(D13PinExt.d13 -> FmcSdRamDataPins<D, D, D, D, D, D, D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U>);
    pin_ext!(D14PinExt.d14 -> FmcSdRamDataPins<D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U>);
    pin_ext!(D15PinExt.d15 -> FmcSdRamDataPins<D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U>);
    pin_ext!(D16PinExt.d16 -> FmcSdRamDataPins<D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U>);
    pin_ext!(D17PinExt.d17 -> FmcSdRamDataPins<D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U>);
    pin_ext!(D18PinExt.d18 -> FmcSdRamDataPins<D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U>);
    pin_ext!(D19PinExt.d19 -> FmcSdRamDataPins<D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U>);
    pin_ext!(D20PinExt.d20 -> FmcSdRamDataPins<D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U>);
    pin_ext!(D21PinExt.d21 -> FmcSdRamDataPins<D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U>);
    pin_ext!(D22PinExt.d22 -> FmcSdRamDataPins<D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U>);
    pin_ext!(D23PinExt.d23 -> FmcSdRamDataPins<D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U>);
    pin_ext!(D24PinExt.d24 -> FmcSdRamDataPins<D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, U, U, U, U, U, U, U>);
    pin_ext!(D25PinExt.d25 -> FmcSdRamDataPins<D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, U, U, U, U, U, U>);
    pin_ext!(D26PinExt.d26 -> FmcSdRamDataPins<D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, U, U, U, U, U>);
    pin_ext!(D27PinExt.d27 -> FmcSdRamDataPins<D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, U, U, U, U>);
    pin_ext!(D28PinExt.d28 -> FmcSdRamDataPins<D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, U, U, U>);
    pin_ext!(D29PinExt.d29 -> FmcSdRamDataPins<D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, U, U>);
    pin_ext!(D30PinExt.d30 -> FmcSdRamDataPins<D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, U>);
    pin_ext!(D31PinExt.d31 -> FmcSdRamDataPins<D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D ,D>);

    pin_ext!(Ba0PinExt.ba0 -> FmcSdRamBankPins<D, U>);
    pin_ext!(Ba1PinExt.ba1 -> FmcSdRamBankPins<D, D>);

    pin_ext!(Nbl0PinExt.nbl0 -> FmcSdRamByteMaskPins<D, U, U, U>);
    pin_ext!(Nbl1PinExt.nbl1 -> FmcSdRamByteMaskPins<D, D, U, U>);
    pin_ext!(Nbl2PinExt.nbl2 -> FmcSdRamByteMaskPins<D, D, D, U>);
    pin_ext!(Nbl3PinExt.nbl3 -> FmcSdRamByteMaskPins<D, D, D, D>);
}