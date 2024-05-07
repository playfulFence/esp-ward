#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use esp_backtrace as _;
use esp_ward::{
    display::{max7219::*, Display},
    peripherals::joystick::Joystick,
};

use embedded_hal::{adc::OneShot, digital::v2::InputPin};
use esp_hal::{
    analog::adc::{AdcPin, ADC},
    gpio::{Analog, GpioPin},
    prelude::*,
};

#[entry]
fn main() -> ! {
    // Max7219 display requires allocator
    esp_ward::prepare_alloc!();
    let peripherals = esp_ward::take_periph!();
    let system = esp_ward::take_system!(peripherals);
    let (_clocks, pins, mut delay) = esp_ward::init_chip!(peripherals, system);

    let mut display = Max7219Display::create_on_pins(
        // Data pin
        pins.gpio17.into_push_pull_output(),
        // Cs pin
        pins.gpio16.into_push_pull_output(),
        // Clk pin
        pins.gpio4.into_push_pull_output(),
        // Amount of displays in chain
        7,
        delay,
    );

    let (mut joystick, mut adc) =
        esp_ward::create_joystick!(peripherals, pins, pins.gpio18.into_pull_up_input());

    let mut x: usize = 1;
    let mut y: usize = 1;

    let mut y_axis_actual :u16 = 0;
    let mut x_axis_actual :u16 = 0;

    display.set_pixel(x, y);

    loop {
        esp_ward::wait!(delay, 40);

        x_axis_actual = joystick.get_x(&mut adc);
        y_axis_actual = joystick.get_y(&mut adc);

        esp_println::println!("x: {}, y: {}", x_axis_actual, y_axis_actual);

        if joystick.select_pressed(delay) {
            display.reset();
        }

        if x_axis_actual < esp_ward::peripherals::joystick::ROUGH_THRESHOLD {
            // right
            x += 1;
            display.set_pixel(x, y);
        }

        if x_axis_actual > esp_ward::peripherals::joystick::ROUGH_THRESHOLD {
            // left
            x -= 1;
            display.set_pixel(x, y);
        }

        if y_axis_actual < esp_ward::peripherals::joystick::ROUGH_THRESHOLD {
            // down
            y += 1;
            display.set_pixel(x, y);
        }

        if y_axis_actual > esp_ward::peripherals::joystick::ROUGH_THRESHOLD {
            // up
            y -= 1;
            display.set_pixel(x, y);
        }
    }
}
