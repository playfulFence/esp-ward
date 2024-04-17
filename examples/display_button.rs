#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use esp_backtrace as _;
use esp_hal::prelude::*;
use esp_ward::{
    display::{ili9341::*, DisplaySegment, EGDisplay},
    peripherals::{button::Button, UnifiedData},
};

#[entry]
fn main() -> ! {
    let peripherals = esp_ward::take_periph!();
    let system = esp_ward::take_system!(peripherals);
    let (clocks, pins, delay) = esp_ward::initialize_chip!(peripherals, system);

    let bus = esp_ward::init_spi_default!(peripherals, pins, clocks);

    cfg_if::cfg_if! {
       if #[cfg(any(
            feature = "esp32",
            feature = "esp32s2",
            feature = "esp32s3",
            feature = "esp32c3",
            feature = "esp32c6",
            feature = "esp32c2",
        ))] {
            let reset_pin = pins.gpio2.into_push_pull_output();
        } else {
            let reset_pin = pins.gpio4.into_push_pull_output();
        }
    }

    cfg_if::cfg_if! {
       if #[cfg(any(
            feature = "esp32",
            feature = "esp32s2",
            feature = "esp32s3",
            feature = "esp32c3",
            feature = "esp32c6",
            feature = "esp32c2",
        ))] {
            let dc_pin = pins.gpio3.into_push_pull_output();
        } else {
            let dc_pin = pins.gpio5.into_push_pull_output();
        }
    }

    let mut display = Ili9341Display::create_on_spi(bus, reset_pin, dc_pin, delay);

    display.write_segment_name(DisplaySegment::Center, "Button status", DEFAULT_STYLE_MID);
    display.write_to_segment(DisplaySegment::Center, "Not pressed", DEFAULT_STYLE_MID);

    let mut button = Button::create_on_pins(pins.gpio26.into_pull_up_input());

    loop {
        while button.read(delay).unwrap() {
            display.write_to_segment(DisplaySegment::Center, "Pressed!", DEFAULT_STYLE_MID);
        }
    }
}
