#[macro_export]
macro_rules! pin_ext {
    ($trait_name:ident<$periph:ident: $periph_map:ident, ..., $($pins:ident),*>.$fn_name:ident -> $type_name:ident<$($periph_pins:ident),+>) => {
        pub trait $trait_name<
            $periph: $periph_map,
            Pin: GpioPinMap,
            Mode: PinModeToken,
            Type: PinTypeToken,
            Pull: PinPullToken,
            $($pins),+
        >
        {
            fn $fn_name(self, pin: GpioPin<Pin, Mode, Type, Pull>) -> $type_name<$($periph_pins),+>;
        }
    };
}

#[macro_export]
macro_rules! pin_impl {
    ($trait_name:ident for $type_name:ident<$periph:path, ...>.$fn_name:ident, $pin:ident, $mode:ty; $($pins_in:ident),* -> $($pins_out:ty),*) => {
        impl<
                Type: PinTypeToken,
                Pull: PinPullToken,
                $($pins_in),+
            > $trait_name<
                $periph,
                $pin,
                $mode,
                Type,
                Pull,
                $($pins_in),+
                // Undefined, Miso, Mosi
            > for $type_name<$periph, $($pins_in),+>
        {
            fn $fn_name(self, _pin: drone_stm32f4_gpio_drv::GpioPin<
                $pin,
                $mode,
                Type,
                Pull,
            >) -> $type_name<$periph, $($pins_out),*> {
                $type_name::new()
            }
        }
    };
}
