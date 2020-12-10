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
The uart driver uses any of the stm32 uart periperals together with their corresponding dma rx/tx streams to achieve asynchronous read and write operations with minimal cpu overhead.

There is a working [Drone OS] example application in the [examples folder](./examples/uart/).
The driver is initially setup like the following:

```rust
thr.usart_2.enable_int();

// TODO: PIN
GpioPinCfg::from(periph_gpio_a2!(reg)) // TX.
        .into_af7()
        .into_pp()
        .with_speed(GpioPinSpeed::VeryHighSpeed);
GpioPinCfg::from(periph_gpio_a3!(reg)) // RX.
    .into_af7()
    .into_pp()
    .with_speed(GpioPinSpeed::VeryHighSpeed);

let dma1 = DmaCfg::init(periph_dma1!(reg));

let setup = UartSetup::usart2(periph_usart2!(reg), thr.usart_2, pclk1);
let uart_drv = UartDrv::init(setup);
```

The usart interrupt is first enabled in the nvic.
The rx/tx pins are then configured, and passed together with the usart peripheral, the usart thread, and the configured periheral clock to the `UartSetup` initialization function, creating a uart `setup` structure. The default setup is `9600/8N1`.
Any other setting can be specified on the public properties of `setup` (the `setup` variable must be declared `mut` in this case).
The setup is finally passed to the driver initialization function.

The rx and tx operation of the driver are completely separated, and each of them needs further initiation before use.

### TX Operation
Completing the setup for tx operation looks like this:
```rust
let tx_setup = DmaChSetup::dma1_ch6_stch4(periph_dma1_ch6!(reg), thr.dma_1_ch_6);
let tx_dma = dma1.init_ch(tx_setup);
let mut tx_drv = uart_drv.init_tx(tx_dma);
```

With `tx_drv` we are now finally able to do some communication.
The uart is not started at this time.
For this we must invoke the `start()` function which returns a guard that stops the transmitter when dropped.
We can start writing to the uart after it is started:

```rust
let mut tx = tx_drv.start();
tx.write(b"Hello World!\n").await;
tx.write(b"Drone OS is awesome!\n").await;
tx.flush().await;
drop(tx);
```

The driver supports multiple subsequent calls to the `write()` function which is an asynchronous call that returns a future that completes _when the dma controller is ready to receive more bytes to be written_.
In this way it is possible to fully saturate the uart even for fast baud rates, without any "spacings" due to a software design with multiple `write()` calls.
In the example this have the effect that the `\n` character after the Hello World! write is completely adjacent to the `D` when expecting the uart tx line.
As a consequence of this design: When `write()` completes this does not mean that the data have actually been sent.
For this we use `flush()` which, when returned tells that all data are completely transmitted at which time it is safe to stop the uart.

### RX Operation
The rx part of the driver is initialized like the following:

```rust
let rx_setup = DmaChSetup::dma1_ch5_stch4(periph_dma1_ch5!(reg), thr.dma_1_ch_5);
let rx_dma = dma1.init_ch(rx_setup);
let mut rx_drv = uart_drv.init_rx(rx_setup);
```

Again, as for the tx part, the driver is not yet ready to receive. For this we need to start the receiver:

```rust
let rx_ring_buf = vec![0; 256].into_boxed_slice();
let mut rx = rx_drv.start(rx_ring_buf);

loop {
  let mut buf = [0; 16];
  match rx.read(&mut buf).await {
    Ok(n) => {
      // Data is available in the slice &buf[..n].
    }
    Err(e) => {
      // The ring buffer has overflowed.
    }
  };
}

drop(rx);
```

The driver is started with the ring buffer `rx_ring_buf`. This buffer is internally assigned to the dma controller, and must be sufficiently large to store received bytes before they are consumed using the `read()` function by the user. `read()` returns a future with the number of bytes read. This number may be smaller than the size of the provided read buffer. It returns immediately if there is _any_ data readily available in the ring buffer, or in the future _as soon as there is data available_. More specifically for `n = read(&buf).await?`, we have `1 <= n <= len(buf)`.
This is not using busy waiting on data to become available, but is achieved internally by registering a [Drone OS] [fiber](https://book.drone-os.com/fibers.html) that completes when any data becomes available in the dma controller and therefore the ring buffer.

The `read()` method may return an error if `read()` is not called fast enough, in which case it can happen that the ring buffer has overflowed since the last call to `read()`.

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
