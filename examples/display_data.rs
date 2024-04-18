#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::fmt::Write as coreWrite;

use esp_backtrace as _;
use esp_hal::prelude::*;
use esp_println::println;
use esp_ward::{
    connectivity::{create_socket, get_timestamp, timestamp_to_hms, weekday_from_timestamp},
    display::{ili9341::*, DisplaySegment, EGDisplay},
    peripherals::{bme280::*, button::Button, HumiditySensor, I2cPeriph, TemperatureSensor},
};
use heapless::String;

#[entry]
fn main() -> ! {
    let peripherals = esp_ward::take_periph!();
    let system = esp_ward::take_system!(peripherals);
    let (clocks, pins, mut delay) = esp_ward::initialize_chip!(peripherals, system);

    let i2c_bus = esp_ward::init_i2c_default!(peripherals, pins, clocks);
    let spi_bus = esp_ward::init_spi_default!(peripherals, pins, clocks);

    let mut display = Ili9341Display::create_on_spi(
        spi_bus,
        pins.gpio10.into_push_pull_output(),
        pins.gpio9.into_push_pull_output(),
        delay,
    );

    let mut sensor = Bme280Sensor::create_on_i2c(i2c_bus, delay).unwrap();

    // Include smoltcp in your project in a similar way like it's included in
    // `esp_ward`
    let mut socket_set_entries: [smoltcp::iface::SocketStorage; 3] = Default::default();

    let (wifi_stack, mut rx_buffer, mut tx_buffer) = esp_ward::init_wifi!(
        "iPhone Kirill",
        "esptesty",
        peripherals,
        system,
        clocks,
        socket_set_entries
    );

    let sock = create_socket(
        &wifi_stack,
        esp_ward::connectivity::WORLDTIMEAPI_IP,
        80,
        &mut rx_buffer,
        &mut tx_buffer,
    );

    let mut timestamp = get_timestamp(sock).unwrap();

    display.write_segment_name(DisplaySegment::TopLeft, "Temperature", DEFAULT_STYLE_MID);
    display.write_segment_name(DisplaySegment::TopRight, "Humidity", DEFAULT_STYLE_MID);
    display.write_segment_name(
        DisplaySegment::Center,
        weekday_from_timestamp(&timestamp),
        DEFAULT_STYLE_SMALL,
    );

    // We'll need it to convert numbers to strings, writable on display
    let mut data: String<32> = String::new();
    let (mut h, mut m, mut s) = timestamp_to_hms(timestamp);
    loop {
        write!(data, "{:2}°C", sensor.get_temperature().unwrap()).expect("write! failed...");
        display.write_to_segment(DisplaySegment::TopLeft, data.as_str(), DEFAULT_STYLE_MID);

        write!(data, "{:2}%", sensor.get_humidity().unwrap()).expect("write! failed...");
        display.write_to_segment(DisplaySegment::TopLeft, data.as_str(), DEFAULT_STYLE_MID);

        write!(data, "{}:{}:{}", h, m, s).expect("write! failed...");

        display.write_to_segment(DisplaySegment::Center, data.as_str(), DEFAULT_STYLE_MID);

        display.write_segment_name(
            DisplaySegment::Center,
            weekday_from_timestamp(&timestamp),
            DEFAULT_STYLE_SMALL,
        );

        esp_ward::wait!(delay, 970);
        timestamp += 1;
        (h, m, s) = timestamp_to_hms(timestamp);
    }
}
