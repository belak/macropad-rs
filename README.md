# macropad-rs

This package is an attempt to build a self-contained [Adafruit Macropad](https://learn.adafruit.com/adafruit-macropad-rp2040) firmware in rust.

## Status

- [x] WS2812 (Neopixel) driver
- [x] SH1106 (OLED) driver
- [x] Read button inputs
- [ ] Rotary encoder driver
- [ ] USB serial support
- [ ] USB keyboard support
- [ ] Migrate to interrupt handlers when possible
- [ ] Add macropad functionality
  - [x] Display something on the screen
  - [x] Control LEDs
  - [ ] Do something with button inputs

## Requirements

- The standard Rust tooling (cargo, rustup) which you can install from https://rustup.rs/
- Toolchain support for the cortex-m0+ processors in the rp2040 (thumbv6m-none-eabi)

## Installation of development dependencies

```
rustup target install thumbv6m-none-eabi
```

## Running

For a debug build

```
cargo run
```

For a release build

```
cargo run --release
```

## License

This project is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
