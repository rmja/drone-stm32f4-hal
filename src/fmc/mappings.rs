use crate::sdrampins::*;
use drone_stm32_map::periph::gpio::pin::*;
use drone_stm32f4_gpio_drv::pin_impl;
use drone_stm32f4_gpio_drv::prelude::*;

pin_impl!(SdclkPinExt  for FmcSdRamPins<...>.sdclk,  GpioG8,  AlternateMode<PinAf12>; U, Sdcke0, Sdcke1, Sdne0, Sdne1, Nras, Ncas, Sdnwe -> D, Sdcke0, Sdcke1, Sdne0, Sdne1, Nras, Ncas, Sdnwe);
pin_impl!(Sdcke1PinExt for FmcSdRamPins<...>.sdcke1, GpioH7,  AlternateMode<PinAf12>; Sdclk, Sdcke0, U, Sdne0, Sdne1, Nras, Ncas, Sdnwe -> Sdclk, Sdcke0, D, Sdne0, Sdne1, Nras, Ncas, Sdnwe);
pin_impl!(Sdne1PinExt  for FmcSdRamPins<...>.sdne1,  GpioH6,  AlternateMode<PinAf12>; Sdclk, Sdcke0, Sdcke1, Sdne0, U, Nras, Ncas, Sdnwe -> Sdclk, Sdcke0, Sdcke1, Sdne0, D, Nras, Ncas, Sdnwe);
pin_impl!(NrasPinExt   for FmcSdRamPins<...>.nras,   GpioF11, AlternateMode<PinAf12>; Sdclk, Sdcke0, Sdcke1, Sdne0, Sdne1, U, Ncas, Sdnwe -> Sdclk, Sdcke0, Sdcke1, Sdne0, Sdne1, D, Ncas, Sdnwe);
pin_impl!(NcasPinExt   for FmcSdRamPins<...>.ncas,   GpioG15, AlternateMode<PinAf12>; Sdclk, Sdcke0, Sdcke1, Sdne0, Sdne1, Nras, U, Sdnwe -> Sdclk, Sdcke0, Sdcke1, Sdne0, Sdne1, Nras, D, Sdnwe);
pin_impl!(SdnwePinExt  for FmcSdRamPins<...>.sdnwe,  GpioH5,  AlternateMode<PinAf12>; Sdclk, Sdcke0, Sdcke1, Sdne0, Sdne1, Nras, Ncas, U -> Sdclk, Sdcke0, Sdcke1, Sdne0, Sdne1, Nras, Ncas, D);
// TODO: Implement SDCKE0 and SDNE0

pin_impl!( A0PinExt for FmcSdRamAddressPins.a0,  GpioF0,  AlternateMode<PinAf12>; U, U, U, U, U, U, U, U, U, U, U, U, U -> D, U, U, U, U, U, U, U, U, U, U, U, U);
pin_impl!( A1PinExt for FmcSdRamAddressPins.a1,  GpioF1,  AlternateMode<PinAf12>; D, U, U, U, U, U, U, U, U, U, U, U, U -> D, D, U, U, U, U, U, U, U, U, U, U, U);
pin_impl!( A2PinExt for FmcSdRamAddressPins.a2,  GpioF2,  AlternateMode<PinAf12>; D, D, U, U, U, U, U, U, U, U, U, U, U -> D, D, D, U, U, U, U, U, U, U, U, U, U);
pin_impl!( A3PinExt for FmcSdRamAddressPins.a3,  GpioF3,  AlternateMode<PinAf12>; D, D, D, U, U, U, U, U, U, U, U, U, U -> D, D, D, D, U, U, U, U, U, U, U, U, U);
pin_impl!( A4PinExt for FmcSdRamAddressPins.a4,  GpioF4,  AlternateMode<PinAf12>; D, D, D, D, U, U, U, U, U, U, U, U, U -> D, D, D, D, D, U, U, U, U, U, U, U, U);
pin_impl!( A5PinExt for FmcSdRamAddressPins.a5,  GpioF5,  AlternateMode<PinAf12>; D, D, D, D, D, U, U, U, U, U, U, U, U -> D, D, D, D, D, D, U, U, U, U, U, U, U);
pin_impl!( A6PinExt for FmcSdRamAddressPins.a6,  GpioF12, AlternateMode<PinAf12>; D, D, D, D, D, D, U, U, U, U, U, U, U -> D, D, D, D, D, D, D, U, U, U, U, U, U);
pin_impl!( A7PinExt for FmcSdRamAddressPins.a7,  GpioF13, AlternateMode<PinAf12>; D, D, D, D, D, D, D, U, U, U, U, U, U -> D, D, D, D, D, D, D, D, U, U, U, U, U);
pin_impl!( A8PinExt for FmcSdRamAddressPins.a8,  GpioF14, AlternateMode<PinAf12>; D, D, D, D, D, D, D, D, U, U, U, U, U -> D, D, D, D, D, D, D, D, D, U, U, U, U);
pin_impl!( A9PinExt for FmcSdRamAddressPins.a9,  GpioF15, AlternateMode<PinAf12>; D, D, D, D, D, D, D, D, D, U, U, U, U -> D, D, D, D, D, D, D, D, D, D, U, U, U);
pin_impl!(A10PinExt for FmcSdRamAddressPins.a10, GpioG0,  AlternateMode<PinAf12>; D, D, D, D, D, D, D, D, D, D, U, U, U -> D, D, D, D, D, D, D, D, D, D, D, U, U);
pin_impl!(A11PinExt for FmcSdRamAddressPins.a11, GpioG1,  AlternateMode<PinAf12>; D, D, D, D, D, D, D, D, D, D, D, U, U -> D, D, D, D, D, D, D, D, D, D, D, D, U);
// TODO: Implement A12

pin_impl!( D0PinExt for FmcSdRamDataPins.d0,  GpioD14, AlternateMode<PinAf12>; U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U -> D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
pin_impl!( D1PinExt for FmcSdRamDataPins.d1,  GpioD15, AlternateMode<PinAf12>; D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U -> D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
pin_impl!( D2PinExt for FmcSdRamDataPins.d2,  GpioD0,  AlternateMode<PinAf12>; D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U -> D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
pin_impl!( D3PinExt for FmcSdRamDataPins.d3,  GpioD1,  AlternateMode<PinAf12>; D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U -> D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
pin_impl!( D4PinExt for FmcSdRamDataPins.d4,  GpioE7,  AlternateMode<PinAf12>; D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U -> D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
pin_impl!( D5PinExt for FmcSdRamDataPins.d5,  GpioE8,  AlternateMode<PinAf12>; D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U -> D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
pin_impl!( D6PinExt for FmcSdRamDataPins.d6,  GpioE9,  AlternateMode<PinAf12>; D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U -> D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
pin_impl!( D7PinExt for FmcSdRamDataPins.d7,  GpioE10, AlternateMode<PinAf12>; D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U -> D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
pin_impl!( D8PinExt for FmcSdRamDataPins.d8,  GpioE11, AlternateMode<PinAf12>; D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U -> D, D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
pin_impl!( D9PinExt for FmcSdRamDataPins.d9,  GpioE12, AlternateMode<PinAf12>; D, D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U -> D, D, D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
pin_impl!(D10PinExt for FmcSdRamDataPins.d10, GpioE13, AlternateMode<PinAf12>; D, D, D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U -> D, D, D, D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
pin_impl!(D11PinExt for FmcSdRamDataPins.d11, GpioE14, AlternateMode<PinAf12>; D, D, D, D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U -> D, D, D, D, D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
pin_impl!(D12PinExt for FmcSdRamDataPins.d12, GpioE15, AlternateMode<PinAf12>; D, D, D, D, D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U -> D, D, D, D, D, D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
pin_impl!(D13PinExt for FmcSdRamDataPins.d13, GpioD8,  AlternateMode<PinAf12>; D, D, D, D, D, D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U -> D, D, D, D, D, D, D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
pin_impl!(D14PinExt for FmcSdRamDataPins.d14, GpioD9,  AlternateMode<PinAf12>; D, D, D, D, D, D, D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U -> D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
pin_impl!(D15PinExt for FmcSdRamDataPins.d15, GpioD10, AlternateMode<PinAf12>; D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U -> D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, D, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
// TODO: Implement the rest...

pin_impl!(Ba0PinExt for FmcSdRamBankPins.ba0, GpioG4, AlternateMode<PinAf12>; U, U -> D, U);
pin_impl!(Ba1PinExt for FmcSdRamBankPins.ba1, GpioG5, AlternateMode<PinAf12>; D, U -> D, D);

pin_impl!(Nbl0PinExt for FmcSdRamByteMaskPins.nbl0, GpioE0, AlternateMode<PinAf12>; U, U, U, U -> D, U, U, U);
pin_impl!(Nbl1PinExt for FmcSdRamByteMaskPins.nbl1, GpioE1, AlternateMode<PinAf12>; D, U, U, U -> D, D, U, U);
// TODO: Implement NBL2 and NBL3