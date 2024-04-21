#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

// embassy related imports
use embassy_executor::Spawner;
use embassy_net::Config;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{embassy, prelude::*, timer::TimerGroup};
use esp_println::println;
use esp_ward::connectivity;
#[main]
async fn main(spawner: Spawner) -> ! {
    let peripherals = esp_ward::take_periph!();
    let system = esp_ward::take_system!(peripherals);
    let (clocks, _pins, _delay) = esp_ward::init_chip!(peripherals, system);

    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);

    embassy::init(&clocks, timer_group0);

    let (wifi_interface, controller) =
        esp_ward::init_wifi!("iPhone Kirill", "esptesty", peripherals, system, clocks);

    // Init network stack
    let config = Config::dhcpv4(Default::default());
    let stack = esp_ward::create_stack!(wifi_interface, config);

    spawner
        .spawn(connectivity::connection(
            controller,
            "iPhone Kirill",
            "esptesty",
        ))
        .ok();
    spawner.spawn(connectivity::net_task(&stack)).ok();

    connectivity::wait_wifi!(stack, config);
    connectivity::get_ip!(stack, config);

    let (mut rx_buffer, mut tx_buffer, mut write_buffer, mut recv_buffer) =
        connectivity::prepare_buffers!();

    // Use this for default connection: https://www.hivemq.com/demos/websocket-client/
    let mut client = connectivity::mqtt_connect_default(
        stack,
        "Your clientID",
        &mut rx_buffer,
        &mut tx_buffer,
        &mut write_buffer,
        &mut recv_buffer,
    )
    .await;

    connectivity::mqtt_subscribe(&mut client, "TopicName").await;

    loop {
        Timer::after(Duration::from_millis(2000)).await;
        connectivity::mqtt_send(&mut client, "TopicName", "data").await;
        let string = connectivity::mqtt_receive(&mut client).await;
        println!("{}", string);
    }
}
