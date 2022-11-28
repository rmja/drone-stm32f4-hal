Examples are bootstrapped using the following set of commands:

```
mkdir exti
cd exti
nix flake init -t "github:drone-os/drone#stm32"
```

Append the `drone-stm32f4-hal` as a dependency in `Cargo.toml` with the required set of features and set the features of the `drone-stm32-map` dependency accordingly.
Append `drone-stm32f4-hal/host` to the `host` feature.

Set correct values of `drone_cortexm` and `drone_stm32_map` in `flake.nix`.

Set flash and memory size in `layout.toml`.