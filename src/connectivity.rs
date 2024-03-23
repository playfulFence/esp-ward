use core::fmt::Write as coreWrite;

use embedded_svc::io::{Read, Write};
use esp_println::println;
#[cfg(feature = "wifi")]
use esp_wifi::{
    current_millis,
    wifi::{WifiDevice, WifiDeviceMode},
    wifi_interface::{Socket, WifiStack},
};
use mqttrust::encoding::v4::Pid;
#[cfg(feature = "wifi")]
use smoltcp::{
    phy::Device,
    wire::{HardwareAddress, IpAddress, Ipv4Address},
};

use crate::tiny_mqtt::TinyMqtt;

pub const WORLDTIMEAPI_IP: &str = "213.188.196.246";
pub const HIVE_MQ_IP: &str = "18.196.194.55";
pub const HIVE_MQ_PORT: u16 = 8884;

const INTERVAL_MS: u64 = 1 * 30 * 1000; // 1 minute interval

#[cfg(any(feature = "esp32", feature = "esp32s2", feature = "esp32s3"))]
#[macro_export]
macro_rules! get_timer {
    ($peripherals:ident, $clocks:ident) => {
        esp_hal::timer::TimerGroup::new($peripherals.TIMG1, &$clocks).timer0
    };
}

#[cfg(not(any(feature = "esp32", feature = "esp32s2", feature = "esp32s3")))]
#[macro_export]
macro_rules! get_timer {
    ($peripherals:ident, $clocks:ident) => {
        esp_hal::systimer::SystemTimer::new($peripherals.SYSTIMER).alarm0
    };
}

#[macro_export]
macro_rules! init_wifi {
    ($ssid:expr, $password:expr, $peripherals:ident, $system:ident, $clocks:ident, $sock_entries:ident) => {{
        let init = esp_wifi::initialize(
            esp_wifi::EspWifiInitFor::Wifi,
            get_timer!($peripherals, $clocks),
            esp_hal::Rng::new($peripherals.RNG),
            $system.radio_clock_control,
            &$clocks,
        )
        .unwrap();

        let wifi = $peripherals.WIFI;
        let (iface, device, mut controller, sockets) =
            esp_wifi::wifi::utils::create_network_interface(
                &init,
                wifi,
                esp_wifi::wifi::WifiStaDevice,
                &mut $sock_entries,
            )
            .unwrap();
        let wifi_stack = esp_wifi::wifi_interface::WifiStack::new(
            iface,
            device,
            sockets,
            esp_wifi::current_millis,
        );

        let client_config =
            esp_wifi::wifi::Configuration::Client(esp_wifi::wifi::ClientConfiguration {
                ssid: $ssid.try_into().unwrap(),
                password: $password.try_into().unwrap(),
                ..Default::default()
            });
        let res = controller.set_configuration(&client_config);
        println!("wifi_set_configuration returned {:?}", res);

        controller.start().unwrap();
        println!("is wifi started: {:?}", controller.is_started());

        println!("Start Wifi Scan");
        let res: Result<
            (heapless::Vec<esp_wifi::wifi::AccessPointInfo, 10>, usize),
            esp_wifi::wifi::WifiError,
        > = controller.scan_n();
        if let Ok((res, _count)) = res {
            for ap in res {
                println!("{:?}", ap);
            }
        }

        println!("{:?}", controller.get_capabilities());
        println!("wifi_connect {:?}", controller.connect());

        // wait to get connected
        println!("Wait to get connected");
        loop {
            let res = controller.is_connected();
            match res {
                Ok(connected) => {
                    if connected {
                        break;
                    }
                }
                Err(err) => {
                    println!("{:?}", err);
                    loop {}
                }
            }
        }
        println!("{:?}", controller.is_connected());

        // wait for getting an ip address
        println!("Wait to get an ip address");
        loop {
            wifi_stack.work();

            if wifi_stack.is_iface_up() {
                println!("got ip {:?}", wifi_stack.get_ip_info());
                break;
            }
        }

        (wifi_stack, [0u8; 1536], [0u8; 1536])
    }};
}

pub fn ip_string_to_parts(ip: &str) -> Result<[u8; 4], &'static str> {
    // can't use heapless::Vec
    let mut parts = [0u8; 4];
    let mut current_part = 0;
    let mut value: u16 = 0; // Use u16 to check for values larger than 255

    for c in ip.trim_end_matches('.').chars() {
        match c {
            '.' => {
                if current_part == 4 {
                    return Err("Too many parts");
                }
                if value > 255 {
                    return Err("Each part must be between 0 and 255");
                }
                parts[current_part] = value as u8;
                current_part += 1;
                value = 0;
            }
            '0'..='9' => {
                value = value * 10 + c.to_digit(10).unwrap() as u16;
                if value > 255 {
                    return Err("Each part must be between 0 and 255");
                }
            }
            _ => return Err("Invalid character in IP address"),
        }
    }

    // Check if last part is valid and assign it
    if current_part != 3 || value > 255 {
        return Err("Invalid IP address format");
    }

    parts[3] = value as u8;

    Ok(parts)
}

pub fn find_unixtime(response: &[u8]) -> Option<u64> {
    // Convert the response to a string slice
    let response_str = core::str::from_utf8(response).ok()?;

    // Look for the "unixtime" key in the response
    let unixtime_key = b"\"unixtime\":";
    if let Some(start) = response_str.find(core::str::from_utf8(unixtime_key).ok()?) {
        // Find the start of the number (skipping the key and any potential spaces)
        let number_start = start + unixtime_key.len();
        let number_end = response_str[number_start..]
            .find(|c: char| !c.is_digit(10) && c != ' ')
            .map_or(response_str.len(), |end| number_start + end);

        // Parse the number
        response_str[number_start..number_end].parse().ok()
    } else {
        None
    }
}

pub fn timestamp_to_hms(timestamp: u64) -> (u64, u64, u64) {
    let seconds_per_minute = 60;
    let minutes_per_hour = 60;
    let hours_per_day = 24;
    let seconds_per_hour = seconds_per_minute * minutes_per_hour;
    let seconds_per_day = seconds_per_hour * hours_per_day;

    let hours = (timestamp % seconds_per_day) / seconds_per_hour;
    let minutes = (timestamp % seconds_per_hour) / seconds_per_minute;
    let seconds = timestamp % seconds_per_minute;

    (hours, minutes, seconds)
}

#[cfg(feature = "wifi")]
pub fn create_socket<'a, 's, MODE>(
    wifi_stack: &'s WifiStack<'a, MODE>,
    ip_string: &str,
    port: u16,
    rx_buffer: &'a mut [u8],
    tx_buffer: &'a mut [u8],
) -> Socket<'s, 'a, MODE>
where
    MODE: WifiDeviceMode,
{
    let mut socket = wifi_stack.get_socket(rx_buffer, tx_buffer);
    socket.work();

    let ip_parts = ip_string_to_parts(ip_string).unwrap();

    match socket.open(
        smoltcp::wire::IpAddress::Ipv4(Ipv4Address::new(
            ip_parts[0],
            ip_parts[1],
            ip_parts[2],
            ip_parts[3],
        )),
        port,
    ) {
        Ok(_) => println!("Socket opened..."),
        Err(e) => panic!("Error opening socket: {:?}", e),
    }

    socket
}

#[cfg(feature = "wifi")]
pub fn send_request<'a, 's, MODE>(socket: &mut Socket<'s, 'a, MODE>, request: &str)
where
    MODE: WifiDeviceMode,
{
    socket.write(request.as_bytes()).unwrap();
    socket.flush().unwrap();
}

#[cfg(feature = "wifi")]
pub fn get_time<'a, 's, MODE>(mut socket: Socket<'s, 'a, MODE>) -> Result<(u64, u64, u64), ()>
where
    MODE: WifiDeviceMode,
{
    let request = "GET /api/timezone/Europe/Prague HTTP/1.1\r\nHost: worldtimeapi.org\r\n\r\n";
    let mut buffer = [0u8; 4096];

    // Using classic "worldtime.api" to get time
    send_request(&mut socket, request);

    let mut total_size = 0usize;

    loop {
        if total_size >= buffer.len() {
            // Buffer is full
            println!("Buffer is full, processed {} bytes", total_size);
            // Here you might want to process the buffer and then clear it
            total_size = 0; // Reset total_size if you wish to reuse the buffer
                            // continue; // Optionally continue reading after processing
            break; // or break if you're done
        }

        let buffer_slice = &mut buffer[total_size..]; // Slice the buffer from the current total_size to the end
        match socket.read(buffer_slice) {
            Ok(0) => {
                // The connection has been closed by the peer
                println!("Connection closed, total read size: {}", total_size);
                break;
            }
            Ok(len) => {
                println!("Read {} bytes", len);
                total_size += len;
                // buffer[..total_size] now contains the data read in this
                // iteration
            }
            Err(e) => {
                println!("Failed to read from socket: {:?}", e);
                break;
            }
        }
    }

    socket.disconnect();

    let wait_end = current_millis() + 5 * 1000;
    while current_millis() < wait_end {
        socket.work();
    }

    if let Some(timestamp) = find_unixtime(&buffer[..total_size]) {
        let mut timestamp = timestamp;
        timestamp += 60 * 60;
        return Ok(timestamp_to_hms(timestamp));
    } else {
        println!("Failed to find or parse the 'unixtime' field.");
        return Err(());
    }
}

// Supposing that received socket is set on HIVE MQ ip and port
#[cfg(feature = "mqtt")]
pub fn mqtt_connect_default<'a, MODE>(
    wifi_stack: &'a WifiStack<'a, MODE>,
    client_id: &'a str,
    write_buffer: &'a mut [u8],
    recv_buffer: &'a mut [u8],
) where
    MODE: WifiDeviceMode,
{
    let socket = create_socket(
        wifi_stack,
        HIVE_MQ_IP,
        HIVE_MQ_PORT,
        recv_buffer,
        write_buffer,
    );

    let mut mqtt = TinyMqtt::new(client_id, socket, esp_wifi::current_millis, None);

    let mut last_sent_millis = 0;
    let mut first_msg_sent = false;

    crate::tiny_mqtt::sleep_millis(1_000);
    println!("Trying to connect");
    mqtt.disconnect().ok();
    let ip_parts: [u8; 4] = ip_string_to_parts(HIVE_MQ_IP).unwrap();
    loop {
        if let Err(e) = mqtt.connect(
            IpAddress::Ipv4(Ipv4Address::new(
                ip_parts[0],
                ip_parts[1],
                ip_parts[2],
                ip_parts[3],
            )),
            HIVE_MQ_PORT,
            10,
            Some("esp32s2"),
            Some("1234567kM".as_bytes()),
        ) {
            println!(
                "Something went wrong ... retrying in 10 seconds. Error is {:?}",
                e
            );
            // wait a bit and try it again
            crate::tiny_mqtt::sleep_millis(10_000);
            continue;
        }
        break;
    }

    println!("Connected to MQTT broker");
    let mut topic_name: heapless::String<32> = heapless::String::new();
    write!(topic_name, "{}/feeds/temperature", "esp32s2").ok();
    println!("Here!");

    let mut pkt_num = 10;
    loop {
        println!("здуся!");
        if mqtt.poll().is_err() {
            println!("error, fuck");
            break;
        }
        println!("zdusya1!");
        println!(
            "{} > {} || {}",
            esp_wifi::current_millis(),
            last_sent_millis + INTERVAL_MS,
            !first_msg_sent,
        );
        println!("zdusya2!");

        if esp_wifi::current_millis() > last_sent_millis + INTERVAL_MS || !first_msg_sent {
            println!("zdusya2!");
            first_msg_sent = true;

            let temperature: f32 = 32.2;

            println!("...");

            let mut msg: heapless::String<32> = heapless::String::new();
            write!(msg, "{}", temperature).ok();
            if mqtt
                .publish_with_pid(
                    Some(Pid::try_from(pkt_num).unwrap()),
                    &topic_name,
                    msg.as_bytes(),
                    mqttrust::QoS::AtLeastOnce,
                )
                .is_err()
            {
                break;
            }

            pkt_num += 1;
            last_sent_millis = esp_wifi::current_millis();
        }
        println!("zdusya2!");
    }

    println!("Disconnecting");
    mqtt.disconnect().ok();
}
