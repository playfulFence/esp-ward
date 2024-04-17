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

    let mut display = Ili9341Display::create_on_spi(
        bus,
        pins.gpio6.into_push_pull_output(),
        pins.gpio7.into_push_pull_output(),
        delay,
    );

    display.write_segment_name(DisplaySegment::Center, "Button status", DEFAULT_STYLE_MID);
    display.write_to_segment(DisplaySegment::Center, "Not pressed", DEFAULT_STYLE_MID);

    let mut button = Button::create_on_pins(pins.gpio10.into_pull_up_input());

    loop {
        while button.read(delay).unwrap() {
            display.write_to_segment(DisplaySegment::Center, "Pressed!", DEFAULT_STYLE_MID);
        }
    }
}
