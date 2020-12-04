![maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# drone-stm32f4-hal

[Drone OS] hardware abstraction layer (HAL) for STM32F4 micro-controllers.

## Usage

Add the crate to your `Cargo.toml` dependencies:

```toml
[dependencies]
drone-stm32f4-hal = { git = "https://github.com/rmja/drone-stm32f4-hal", features = ["uart"] }
```

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
