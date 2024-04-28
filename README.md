# esp-ward: A Rust Library for Embedded Sensor Applications on ESP32

[![Web-Page](https://img.shields.io/website-up-down-green-red/http/shields.io.svg)](https://playfulfence.github.io/esp-ward/)

Welcome to the [`esp-ward`](https://playfulfence.github.io/esp-ward/) official repository!

## About the Project

`esp-ward` is an extensible Rust crate created for simplified the development of embedded sensor applications on the ESP32 platform. This library provides simplified API for basic operations with various peripherals.

### Key Features

*   **User-friendly Sensor Management**: Provides an intuitive interface for the effortless setup, activation, and tracking of sensors, ideal for smart home or automation processes.
*   **Modular and Extensible Architecture**: Designed with flexibility in mind, allowing easy integration and expansion to accommodate diverse application requirements.
*   **Created for [`Espressif`](https://www.espressif.com) chips**: Built on Rust's robust safety features and performance, this crate is built on top of [`esp-hal`](https://github.com/esp-rs/esp-hal) driver for various `esp` chips

### Supported Functionality

| Category         | Devices                            |
|------------------|------------------------------------|
| Connectivity     | Wi-Fi, MQTT                        |
| Temperature      | AHT20, BME280                      |
| Humidity         | AHT20, BME280                      |
| Pressure         | BME280                             |
| Motion Sensors   | PIR Sensor                         |
| Distance Sensors | HC-SR04 Ultrasonic Sensor          |
| Light Sensors    | TSL2591                            |
| Gas Sensors      | SGP30 (CO2 and VOC)                |
| User Input       | Generic Button, Joystick           |
| Displays         | ILI9341, MAX7219, PCD8544          |

## Getting Started

### Prerequisites

Before you begin, ensure you have the `MSRV` and the necessary tools and environment for ESP32+Rust development installed on your system (check [The Rust on ESP book](https://docs.esp-rs.org/book/) from our `esp-rs` team).

> MSRV: 1.76.0.1

### Installation

To include `esp-ward` in your project, add the following to your `Cargo.toml` file:
```toml
esp-ward = { git = "https://github.com/playfulFence/esp-ward.git", features = ["required", "features"]}
```

For features guide check [this section](#how-to-build-and-example)

## Usage

#### How to build and example? 
If you want to try some default examples, you need to fork this repo and from the root directory of this project execute this CLI command: 

```bash
cargo espflash flash --example=<example-name> --features=<chip-feature> --target=<target> --monitor
```

Where: 
1) `example-name` - name of your example (`display_data`, `led_scrolling`)
2) `chip-feature` - depending on what chip and example you use, you need to set correct features, here's some minimalist guide: 
    - Decide which chip are you using
    - If chosen example utilizes `wifi`-related features - enable corresponding feature (example: `esp32s2-wifi`), same works with `mqtt` functionalities. In case you want to use something, which requires `allocator` (in this example list - if you want to use `max7219` display) - make sure to enable `alloc` feature (example: `--features=esp32s2,alloc`). In case you need just basic functionality - just write a name of required chip to a feature list (example: `--features=esp32s2`)
3) Target lets the `espflash` know, which architecture your chip is using:
    - `xtensa-<chip>-none-elf` - for Xtensa-based chips
    - `riscv32imc-unknown-none-elf` - for `esp32c3`
    - `riscv32imac-unknown-none-elf` - for `esp32c6` and `esp32h2`

#### How to use this crate?
The library provides a straightforward approach to managing sensors:

```rust

// Example instantiation of a sensor 
let peripherals = esp_ward::take_periph!();
let system = esp_ward::take_system!(peripherals);
let (clocks, pins, mut delay) = esp_ward::init_chip!(peripherals, system);

let bus = esp_ward::init_i2c_default!(peripherals, pins, clocks);
let mut sensor = Aht20Sensor::create_on_i2c(bus, delay).unwrap();

println!("Temperature {}", sensor.get_temperature().unwrap())
```

Detailed examples for various use cases can be found in the `/examples` directory.

### Troubleshooting: 
- If something is wrong with a building process and `CI` is green after latest commit to `esp-ward` - problem is on your side
    - Check if you correcty installed the toolchains and the rest of environment (check [`The Rust on ESP Book`](https://docs.esp-rs.org/book/))
    - Make sure to use [`espup`](https://github.com/esp-rs/espup)
    - Try to pass correct toolchain to `cargo` (`esp` for Xtensa, `nightly` for `RISC`) - `cargo +<toolchain> espflash flash...`
- On Xtensa targets you might encounter issues with linker. Try to install and export the [`esp-idf`](https://github.com/espressif/esp-idf) 

- Report an `issue` to this repo or open a `Pull Request` fixing it!

## Documentation
Comprehensive documentation in typical `Rust docs` fomat is available [web-page](https://playfulfence.github.io/esp-ward/) of a driver, in "`Docs`" section

## Contributing
Contributions to `esp-ward` are welcome and appreciated. Please refer to the [CONTRIBUTING.md](CONTRIBUTING.md) file for guidelines on how to submit issues, follow the pull request process, and code of conduct.

## Future Enhancements

The ongoing development of `esp-ward` seeks to introduce new features, optimizations, and broader sensor, display and connectivity features support.

## License

This project is released under the Apache License 2.0, which allows for both personal and commercial use, modification, and distribution.
