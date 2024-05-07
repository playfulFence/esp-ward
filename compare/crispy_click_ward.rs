#![no_std]
#![no_main]

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
    let (clocks, pins, mut delay) = esp_ward::init_chip!(peripherals, system);

    let bus = esp_ward::init_spi_default!(peripherals, pins, clocks);

    let mut display = Ili9341Display::create_on_spi(
        bus,
        pins.gpio6.into_push_pull_output(),
        pins.gpio7.into_push_pull_output(),
        delay,
    );

    display.write_segment_name(DisplaySegment::TopLeft, "Blue", DEFAULT_STYLE_MID);
    display.write_to_segment(DisplaySegment::TopLeft, "Not pressed", DEFAULT_STYLE_MID);

    display.write_segment_name(DisplaySegment::BottomRight, "Green", DEFAULT_STYLE_MID);
    display.write_to_segment(DisplaySegment::BottomRight, "Not pressed", DEFAULT_STYLE_MID);

    let mut button_green = Button::create_on_pins(pins.gpio10.into_pull_up_input());
    let mut button_blue = Button::create_on_pins(pins.gpio1.into_pull_up_input());

    let mut changed_blue: bool = false;
    let mut changed_green: bool = false;

    loop {
        while button_blue.read(delay).unwrap() == true {
            display.write_to_segment(DisplaySegment::TopLeft, "Pressed", DEFAULT_STYLE_MID);
            esp_ward::wait!(delay, 1000);
            changed_blue = true;
        }

        while button_green.read(delay).unwrap() == true {
            display.write_to_segment(DisplaySegment::BottomRight, "Pressed", DEFAULT_STYLE_MID);
            esp_ward::wait!(delay, 1000);
            changed_green = true;
        }

        if changed_blue {
            display.write_to_segment(DisplaySegment::TopLeft, "Not pressed", DEFAULT_STYLE_MID);
            changed_blue = false;
        }
        if changed_green {
            display.write_to_segment(DisplaySegment::BottomRight, "Not pressed", DEFAULT_STYLE_MID);
            changed_green = false;
        }
    }
}
