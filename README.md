![maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# drone-stm32f4-hal

[Drone OS] hardware abstraction layer (HAL) for STM32F4 micro-controllers.

## Usage

Add the crate to your `Cargo.toml` dependencies:

```toml
[dependencies]
drone-stm32f4-hal = { git = "https://github.com/rmja/drone-stm32f4-hal", features = ["..."] }
```

A feature maps to a peripheral driver. There are the following low-level drivers:
* `rcc` Type safe clock configuration model, and a rcc, pwr, and flash driver.
* `gpio` Type safe pin-setup configuration primitives.
* `dma` Dma primitives for uniform configuration across other, dma-dependent drivers.

There are the following more high-level drivers:

* `exti` An external pin-interrupt driver.
* `spi` Dma driven, future based spi driver.
* `uart` Dma driven, future based uart driver.
* `fmc` External SDRAM driver.

## RCC
A necessary but often complicated task when starting a new embedded application is to correctly configure the various clocks within the mcu. The `rcc` feature contains two parts:
1. A clock tree configuration _model_, and
2. a set of _drivers_ that effectuate the model on boot.

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
let pwr = Pwr::with_enabled_clock(periph_pwr!(reg));
let flash = Flash::new(periph_flash!(reg));

let hseclk = rcc.stabilize(consts::HSECLK).await;
let pll = rcc.select(consts::PLLSRC_HSECLK, hseclk).stabilize(consts::PLL).await;
let hclk = rcc.configure(consts::HCLK);
let pclk1 = rcc.configure(consts::PCLK1);
let pclk2 = rcc.configure(consts::PCLK2);
pwr.enable_overdrive();
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

## GPIO
The `gpio` feature includes a set of types that makes it easy and safe to configure ports and their respective pins.
Consider the following example that configures pin `A5` into alternate-function mode, with push/pull type, and for high speed operation.

```rust
use drone_stm32f4_hal::gpio::{prelude::*, GpioHead};

let gpio_a = GpioHead::with_enabled_clock(periph_gpio_a_head!(reg));
let pin_sck = gpio_a.pin(periph_gpio_a5!(reg))
  .into_alternate()
  .into_pushpull()
  .with_speed(GpioPinSpeed::VeryHighSpeed);
```
The `pin_sck` has type `GpioPin<GpioA5, AlternateMode<Af>, PushPullType, NoPull>` where `Af` is any of the alternate function marker types `PinAf0,...,PinAf15`.
It is not needed to explicitly specify the alternate function,
as it is included in the function prototypes for any drivers that uses a pin, and so its type information flows "backwards" into the configuration of the pin.

The clock for `gpio_a` is enabled when it is constructed as the clock is required when configuring the gpio.
The clock should be explicitly disabled if so desired. This is unsafe, as special care should be taken if operating with disabled gpio clocks:

```rust
unsafe {
  port_a.disable_clock(); // Explicitly disable clock - use with care!
}
```

## DMA
The `dma` driver is simple, but includes type safety features for other drivers.
Consider the following lines of code from the [uart example](./examples/uart/src/thr/root.rs):

```rust
let dma1 = DmaCfg::with_enabled_clock(periph_dma1!(reg));
let rx_dma = dma1.ch(DmaChSetup::new(periph_dma1_ch5!(reg), thr.dma_1_ch_5));
```
`rx_dma` has type `DmaChCfg<Dma1Ch5, DmaStCh, DmaInt>` where `Dma1Ch5` means "DMA1 _stream_ 5".
ST has changed the nomenclature for newer mcu's, and [Drone OS] uses the new naming scheme.
The F4 series operate with _streams_ and _channels_.
We call the channels within a stream a "stream channel" to avoid confusion.

`DmaStCh` may be any of `DmaStCh0,...,DmaStCh7` as there 8 possible stream channels within a stream.
As for the alternate function mode in the gpio driver,
the actual stream channel is not explicitly specified,
as this information flows back into the type of `rx_dma` when the variable is actually used.

## EXTI
The exti driver is similar to the [smartoris-exti] driver,
but with type safety from the gpio pin configuration ensuring that interrupts for a given pin is configured on the correct exti peripheral.
There is a working [Drone OS] example application in the [examples folder](./examples/exti/):


```rust
thr.exti_2.enable_int();

let syscfg = Syscfg::with_enabled_clock(periph_syscfg!(reg));
let exti = ExtiDrv::new(periph_exti2!(reg), thr.exti_2, &syscfg).into_rising_edge();

let gpio = GpioHead::with_enabled_clock(periph_gpio_i_head!(reg));
let pin = gpio.pin(periph_gpio_i2!(reg)).into_input().into_pushpull().into_pulldown();

let line = exti.line(&pin);

let stream = line.create_saturating_stream();
exti.listen();

while let Some(tick) = stream.next().await {
  // Rising edge was triggered
}
```

The exti interrupt is first enabled in the nvic. The exti driver is then initialized, after which the pin is configured as input.
The exti driver provides the `line()` function returning an `ExtiLine` struct, from which one can create a stream of events.
We start to listen when the stream is ready to consume the interrupts.

## SPI
The spi driver provides future based spi transfers using dma.
There is a working [Drone OS] example application in the [examples folder](./examples/spi/).
The driver is initially setup like the following:

```rust
thr.spi_1.enable_int();
thr.dma_2_ch_2.enable_int();
thr.dma_2_ch_3.enable_int();

let gpio_a = GpioHead::with_enabled_clock(periph_gpio_a_head!(reg));
let pin_sck = gpio_a.pin(periph_gpio_a5!(reg))
  .into_alternate()
  .into_pushpull()
  .with_speed(GpioPinSpeed::VeryHighSpeed);
let pin_miso = gpio_a.pin(periph_gpio_a6!(reg))
  .into_alternate()
  .into_pushpull()
  .with_speed(GpioPinSpeed::VeryHighSpeed);
let pin_mosi = gpio_a.pin(periph_gpio_a7!(reg))
  .into_alternate()
  .into_pushpull()
  .with_speed(GpioPinSpeed::VeryHighSpeed);

let dma2 = DmaCfg::with_enabled_clock(periph_dma2!(reg));
let miso_dma = dma2.ch(DmaChSetup::new(periph_dma2_ch2!(reg), thr.dma_2_ch_2));
let mosi_dma = dma2.ch(DmaChSetup::new(periph_dma2_ch3!(reg), thr.dma_2_ch_3));

let pins = SpiPins::default().sck(pin_sck).miso(pin_miso).mosi(pin_mosi);
let setup = SpiSetup::new(
    periph_spi1!(reg),
    thr.spi_1,
    pins,
    pclk2,
    BaudRate::Max(7_700_000),
);
let spi = SpiDrv::init(setup).into_master(miso_dma, mosi_dma);
```

The spi interrupt is first enabled in the nvic before the spi pins are configured.
The base driver does not handle chip selection, and it is up to the user to correctly select the desired chip before transferring on the spi bus (see below).
The dma and its channels matching the used spi peripheral are then configured.
A `SpiSetup` structure containing all the parameters for the driver are created with the spi pins to verify that the pins actually map to the spi peripheral. The peripheral clock is also specified together witht the maximum allowed baud rate.

### Chip Selection
Chip selection is not an integrated part of the spi driver, but a small chip controller shim is included in the driver.
It handles simple select/deselect like the following:

```rust
use drone_stm32f4_hal::spi::chipctrl::*;

let pin_cs = gpio_b.pin(periph_gpio_b7!(reg))
  .into_output()
  .with_speed(GpioPinSpeed::HighSpeed);

let mut chip = SpiChip::new_deselected(pin_cs);
let selection = spi_master.select(&mut chip);
// Do some communication...
drop(selection); // drop() deselects chip.
```

It extends the spi master driver with the `select()` method which returns a guard that deselects the chip when dropped.

### Communication

The communication can be done using the three methods `write()`, `read()`, and `xfer()`:

```rust
let selection = spi_master.select(&mut chip);
let tx_buf = [1, 2, 3, 4].as_ref();
let mut rx_buf = [0;4];
spi_master.write(tx_buf).root_wait();
spi_master.read(&mut rx_buf).root_wait();
spi_master.xfer(tx_buf, &mut rx_buf).root_wait();
drop(selection); // drop() deselects chip.
```

The `write()` function simply writes the buffer and discards all bytes received during the write,
the `read()` method emits `0` on the spi bus to "clock out" the selected chip. The received bytes are written to the provided buffer.
`xfer()` performs a full duplex transfer (the two buffer slices must have the same size). 


## UART
The uart driver uses any of the stm32 uart periperals together with their corresponding dma rx/tx streams to achieve asynchronous read and write operations with minimal cpu overhead.

There is a working [Drone OS] example application in the [examples folder](./examples/uart/).
The driver is initially setup like the following:

```rust
thr.usart_2.enable_int();
thr.dma_1_ch_5.enable_int();
thr.dma_1_ch_6.enable_int();

let gpio_a = GpioHead::with_enabled_clock(periph_gpio_a_head!(reg));
let pin_tx = gpio_a.pin(periph_gpio_a2!(reg))
  .into_alternate()
  .into_pushpull()
  .with_speed(GpioPinSpeed::VeryHighSpeed);
let pin_rx = GpioPin::from(periph_gpio_a3!(reg))
  .into_alternate()
  .into_pushpull()
  .with_speed(GpioPinSpeed::VeryHighSpeed);

let dma1 = DmaCfg::with_enabled_clock(periph_dma1!(reg));
let rx_dma = dma1.ch(DmaChSetup::new(periph_dma1_ch5!(reg), thr.dma_1_ch_5));
let tx_dma = dma1.ch(DmaChSetup::new(periph_dma1_ch6!(reg), thr.dma_1_ch_6));

let uart_pins = UartPins::default().tx(pin_tx).rx(pin_rx);

let setup = UartSetup::init(periph_usart2!(reg), thr.usart_2, pclk1);
let uart_drv = UartDrv::init(setup);
```

The usart interrupt is first enabled in the nvic.
The rx/tx pins are then configured together with the dma's.
A `uart_pins` variable is created that is used later when initializing the rx/tx operation.
It should be noted that it is the assigned `uart_pins` that ultimately decides the alternate function of the two pins.
The usart peripheral, the usart thread, and the configured periheral clock to the `UartSetup` initialization function, creating a uart `setup` structure. The default setup is `9600/8N1`.
Any other setting can be specified on the public properties of `setup` (the `setup` variable must be declared `mut` in this case).
The setup is finally passed to the driver initialization function.

The rx and tx operation of the driver are completely separated, and each of them needs further initiation before use.

### TX Operation
Completing the setup for tx operation looks like this:
```rust
let tx_setup = DmaChSetup::init(periph_dma1_ch6!(reg), thr.dma_1_ch_6);
let mut tx_drv = uart_drv.into_tx(tx_dma, &uart_pins);
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
let rx_setup = DmaChSetup::init(periph_dma1_ch5!(reg), thr.dma_1_ch_5);
let mut rx_drv = uart_drv.into_rx(rx_setup, &uart_pins);
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

### TX and RX Operation
The two previous examples have shown tx-only and rx-only operation. One can split the driver into both a tx and rx driver as follows:

```rust
let tx_setup = DmaChSetup::init(periph_dma1_ch6!(reg), thr.dma_1_ch_6);
let rx_setup = DmaChSetup::init(periph_dma1_ch5!(reg), thr.dma_1_ch_5);
let (mut tx_drv, mut rx_drv) = uart_drv.into_trx(tx_setup, rx_setup, &uart_pins);
```

## FMC
The `fmc` feature provides an sdram driver. The driver ensures that all required pins are configured correctly, after which one can obtain a `&mut [T]` slice of the address mapped memory.
There is an [example app](./examples/fmc/src/tasks/root.rs) where the majority of the configuration is setting up pins into the alternate function mode.
After that, the initialization and use of the driver is straight forward:

```rust
let sdram_pins = FmcSdRamPins::default()
  .sdclk(gpio_g.pin(periph_gpio_g8!(reg)).into_alternate().with_speed(GpioPinSpeed::HighSpeed))
  ...;
let address_pins = FmcSdRamAddressPins::default()
  .a0(gpio_f.pin(periph_gpio_f0!(reg)).into_alternate().with_speed(GpioPinSpeed::HighSpeed))
  ...;
let data_pins = FmcSdRamDataPins::default()
  .d0(gpio_d.pin(periph_gpio_d14!(reg)).into_alternate().with_speed(GpioPinSpeed::HighSpeed))
  ...;
let bank_pins = FmcSdRamBankPins::default()
  .ba0(gpio_g.pin(periph_gpio_g4!(reg)).into_alternate().with_speed(GpioPinSpeed::HighSpeed))
  ...;
let mask_pins = FmcSdRamByteMaskPins::default()
  .nbl0(gpio_e.pin(periph_gpio_e0!(reg)).into_alternate().with_speed(GpioPinSpeed::HighSpeed))
  ...;

let fmc = FmcDrv::init_sdram(SdRamSetup::new_bank2(periph_fmc!(reg), consts::SDRAM_CFG, hclk), sdram_pins, address_pins, data_pins, bank_pins, mask_pins);
let ram = fmc.bank2_slice::<usize>();

for i in 0..ram.len() {
    ram[i] = i;
}

for i in 0..ram.len() {
    assert_eq!(i, ram[i], "SDRAM sanity check error!");
}
```
The driver ensures that the correct number of pins is mapped corresponding to the sdram `consts::SDRAM_CFG` [configuration parameters](./examples/fmc/src/consts.rs),
and that they are set into alternate function mode.

## Supported Devices

| stm32_mcu |
|-----------|
| stm32f401 |
| stm32f405 |
| stm32f407 |
| stm32f410 |
| stm32f411 |
| stm32f412 |
| stm32f413 |
| stm32f427 |
| stm32f429 |
| stm32f446 |
| stm32f469 |

## References

* [STM32F429 PM0090 reference manual](https://www.st.com/resource/en/reference_manual/dm00031020.pdf)
* [stm32f4-hal](https://github.com/stm32-rs/stm32f4xx-hal)
* [smartoris-exti]

[Drone OS]: https://www.drone-os.com/
[smartoris-exti]: https://github.com/smartoris/smartoris-exti

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
