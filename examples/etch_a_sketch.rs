#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use esp_backtrace as _;
use esp_hal::prelude::*;
use esp_println::println;
use esp_ward::{
    display::{max7219::*, Display},
    peripherals::joystick::Joystick,
};

#[entry]
fn main() -> ! {
    // Max7219 display requires allocator
    esp_ward::prepare_alloc!();
    let peripherals = esp_ward::take_periph!();
    let system = esp_ward::take_system!(peripherals);
    let (_clocks, pins, mut delay) = esp_ward::initialize_chip!(peripherals, system);

    let mut display = Max7219Display::create_on_pins(
        pins.gpio2.into_push_pull_output(),
        pins.gpio3.into_push_pull_output(),
        pins.gpio1.into_push_pull_output(),
        4,
        delay,
    );

    let (mut joystick, mut adc) =
        esp_ward::create_joystick!(peripherals, pins, pins.gpio9.into_pull_up_input());

    display.write_str("Draw!");

    esp_ward::wait!(delay, 2000);

    display.reset();

    let mut x: usize = 1;
    let mut y: usize = 1;

    display.set_pixel(x, y);

    loop {
        if joystick.select_pressed(delay) {
            display.reset();
        }

        if joystick.get_x(&mut adc) < esp_ward::peripherals::joystick::ROUGH_THRESHOLD {
            // right
            x += 1;
            display.set_pixel(x, y);
        }

        if joystick.get_x(&mut adc) > esp_ward::peripherals::joystick::ROUGH_THRESHOLD {
            // left
            x -= 1;
            display.set_pixel(x, y);
        }

        if joystick.get_y(&mut adc) < esp_ward::peripherals::joystick::ROUGH_THRESHOLD {
            // down
            y += 1;
            display.set_pixel(x, y);
        }

        if joystick.get_y(&mut adc) > esp_ward::peripherals::joystick::ROUGH_THRESHOLD {
            // up
            y -= 1;
            display.set_pixel(x, y);
        }
    }
}
