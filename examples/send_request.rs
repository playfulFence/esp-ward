#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::fmt::Write as coreWrite;

use esp_backtrace as _;
use esp_hal::prelude::*;
use esp_println::println;
use esp_ward::connectivity;
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

    let mut sock = connectivity::create_socket(
        &wifi_stack,
        "142.250.188.243",
        80,
        &mut rx_buffer,
        &mut tx_buffer,
    );

    connectivity::send_request(&mut sock, "GET / HTTP/1.0\r\nHost: www.mobile-j.de\r\n\r\n");
    let (responce, size) = connectivity::get_responce(sock).unwrap();

    // Covert bytes to str
    println!("{}", unsafe { core::str::from_utf8_unchecked(&responce[..size])});
    loop {
    }
}