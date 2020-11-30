![maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# drone-stm32f4-hal

[Drone OS] hardware abstraction layer (HAL) for STM32F4 micro-controllers.

## Usage

Add the crate to your `Cargo.toml` dependencies:

```toml
[dependencies]
drone-stm32f4-hal = { git = "https://github.com/rmja/drone-stm32f4-hal" }
```

### Uart Driver

Example of usage:

```rust
let mut drv = UartDrv::init(...);
{
    let mut tx = drv.tx(); // Enables uart TX peripheral

    let buf1 = vec![0x55, 0xAA, 0x55, 0xAA].into_boxed_slice();
    let buf2 = vec![0x33, 0xEE, 0x33, 0xEE].into_boxed_slice();
    tx.write(&buf1).await; // Future returns as soon as a new write is possible
    tx.write(&buf2).await;
    tx.flush().await; // Wait for uart peripheral to complete transmission
} // Dropping tx guard disables TX on the uart - this is a busy wait if flush() is not called prior to drop
```

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