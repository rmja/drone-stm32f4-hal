![maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# drone-stm32f4-hal

[Drone OS] hardware abstraction layer (HAL) for STM32F4 micro-controllers.

## Usage

Add the crate to your `Cargo.toml` dependencies:

```toml
[dependencies]
drone-stm32f4-hal = { git = "https://github.com/rmja/drone-stm32f4-hal", features = ["..."] }
```

The list of features is currently the following:

* `gpio` A set of type safe pin-setup configuration utilities.
* `rcc` A type safe clock configuration model, and a rcc, pwr, and flash driver.
* `spi` A dma driven, future based spi driver.
* `uart` A dma driven, future based uart driver.

## GPIO
...

## RCC
A necessary but often complicated task when starting a new embedded application is to correctly configure the various clocks within the mcu. The `rcc` features contains two parts: 1) A clock tree configuration _model_, and 2) a set of _drivers_ that effectuate the model on boot.

The following is the model configuration example from the [uart example app](https://github.com/rmja/drone-stm32f4-hal/blob/master/examples/uart/src/consts.rs):

```rust
use drone_stm32f4_hal::rcc::clktree::*;

pub const HSECLK: HseClk = HseClk::new(8_000_000);
pub const PLLSRC_HSECLK: PllSrcMuxSignal = PllSrcMuxSignal::Hse(HSECLK);
pub const PLL: Pll = PLLSRC_HSECLK.to_pllsrc(8).to_pll(360, 2, 8);
pub const SYSCLK_PLL: SysClkMuxSignal = SysClkMuxSignal::Pll(PLL.p);
pub const SYSCLK: SysClk = SYSCLK_PLL.to_sysclk();
pub const HCLK: HClk = SYSCLK.to_hclk(1);
pub const PCLK1: PClk1 = HCLK.to_pclk1(4);
pub const PCLK2: PClk2 = HCLK.to_pclk2(2);
```

The model desribes the following configuration:
* The mcu board is equipped with an 8MHz crystal wired to the mcu, and we want to use it as the high-speed oscillator (HSE).
* The HSE should be used as signal source for the internal PLL.
* The PLL has two output signals: `PLL_P` and `PLL_Q`. With the HSE signal source, configure the pll outputs with the parameters `m`, `n`, `p`, and `q` as follows:
  * `PLL_P = ((HSECLK / 8) * 360) / 2 = 180MHz`
  * `PLL_Q = ((HSECLK / 8) * 360) / 8 = 45MHz`
* Use the PLL's `PLL_P` clock signal as source for the sysclk.
* Define `SYSCLK` to be equal to the selected sysclk signal. This means that this model will run the mcu at 180MHz.
* Let the AHB bus, memory, dma, etc. sourced with `HCLK=SYSCLK/1`, i.e. run at the same frequency with a prescaler of 1.
* Let the low speed peripheral bus run at `PCLK1=HCLK/4=45MHz`.
* Let the high speed peripheral bus run at `PCLK2=HCLK/2=90MHz`.

Note that the configuration is type safe and that parameters are verified at compile time, ensuring that only valid parameters are passed to the RCC configuration registers, and that it is guaranteed that the mcu is not overclocked.

The model is applied as follows ([again from the uart example app](https://github.com/rmja/drone-stm32f4-hal/blob/master/examples/uart/src/tasks/root.rs)):
```rust
use drone_stm32f4_hal::rcc::{
  periph_flash,
  periph_pwr,
  periph_rcc,
  traits::*,
  Flash, Pwr, Rcc, RccSetup,
};

thr.rcc.enable_int(); 

let rcc_setup = RccSetup {
  rcc: periph_rcc!(reg),
  rcc_int: thr.rcc,
};
let rcc = Rcc::init(rcc_setup);
let pwr = Pwr::init(periph_pwr!(reg));
let flash = Flash::init(periph_flash!(reg));

let hseclk = rcc.stabilize(consts::HSECLK).await;
let pll = rcc.select(consts::PLLSRC_HSECLK, hseclk).stabilize(consts::PLL).await;
let pclk1 = rcc.configure(consts::PCLK1);
let pclk2 = rcc.configure(consts::PCLK2);
pwr.enable_od();
flash.set_latency(consts::HCLK.get_wait_states(VoltageRange::HighVoltage));
swo::flush();
swo::update_prescaler(consts::HCLK.f() / log::baud_rate!() - 1);
rcc.select(consts::SYSCLK_PLL, pll.p());
```

A lot is going on here, so one thing at a time. We first enable rcc interupt in the NVIC and then we initialize the `rcc`, `pwr`, and `flash` drivers from their respective [peripheral mappings](https://book.drone-os.com/periph.html).

When the drivers are ready, we can start applying our model:
We start by _stabilizing_ the `HSE` clock, meaning that 1) we configure it according to the `HSECLK` model configuration, and 2) we start it. Stabilizing a clock is an asynchronous operation, so we need to await its completion. The stabilization returns a `ConfiguredClk<HseClk>` where `ConfiguredClk<Clk>` is a zero-overhead indicator that tells us that the clock is ready to use (recall that `consts::HSECLK` is of type `HseClk`).

Moving on, we can now select the `HSE` clock signal as the source for our PLL. The `select()` method requires a `ConfiguredClk<Clk>` so we are guaranteed that the source clock has actually stabilized prior to selecting it as input. Only after that has happened are we able to stablizie the PLL and wait for it to become ready. After this, we can easily configure the peripheral clocks (This could have been done at any time, so there is no need to inforce any guarantees on prior stabilization of clocks).

The next couple of lines enables over-drive (available in e.g. stm32f429) for high-speed operation, sets the correct flash latency for the mcu in the specified voltage range, configures the swo for [logging](https://book.drone-os.com/bluepill-blink/full-speed.html). Lastly we are ready to select the PLL's `PLL_P` output as the source for the sysclk, effectively setting the desired 180MHz mcu speed.

## SPI
There are currently a few bugs. The driver is not complete.

## UART

* [Uart driver echo example](./examples/uart/src/tasks/root.rs)

## References

* [STM32F429 PM0090 reference manual](https://www.st.com/resource/en/reference_manual/dm00031020.pdf)
* [stm32f4-hal](https://github.com/stm32-rs/stm32f4xx-hal)

[Drone OS]: https://www.drone-os.com/

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
