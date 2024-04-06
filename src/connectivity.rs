//! # WiFi Connectivity
//!
//! This module provides functionality for initializing and managing WiFi
//! connections on ESP devices, including MQTT messaging
#[cfg(feature = "mqtt")]
use core::fmt::Write as coreWrite;

#[cfg(feature = "mqtt")]
use embassy_net::{dns::DnsQueryType, tcp::TcpSocket, Stack};
#[cfg(feature = "mqtt")]
use embassy_time::{Duration, Timer};
#[cfg(feature = "wifi")]
use embedded_svc::io::{Read, Write};
#[cfg(feature = "mqtt")]
use embedded_svc::wifi::{ClientConfiguration, Configuration};
#[cfg(feature = "wifi")]
use esp_println::println;
#[cfg(feature = "wifi")]
use esp_wifi::{
    current_millis,
    wifi::{WifiController, WifiDevice, WifiDeviceMode, WifiEvent, WifiStaDevice, WifiState},
    wifi_interface::{Socket, WifiStack},
};
#[cfg(feature = "mqtt")]
use heapless::String;
#[cfg(feature = "mqtt")]
use rust_mqtt::{
    client::{client::MqttClient, client_config::ClientConfig},
    packet::v5::reason_codes::ReasonCode,
    utils::rng_generator::CountingRng,
};
#[cfg(feature = "wifi")]
use smoltcp::wire::{IpAddress, Ipv4Address};

/// Represents the IP address for the WorldTime API server.
pub const WORLDTIMEAPI_IP: &str = "213.188.196.246";
/// Represents the IP address for the HiveMQ MQTT broker.
pub const HIVE_MQ_IP: &str = "18.196.194.55";
/// Represents the port number for the HiveMQ MQTT broker.
pub const HIVE_MQ_PORT: u16 = 8884;

#[cfg(feature = "mqtt")]
#[macro_export]
/// Macro to prepare buffers with fixed sizes for MQTT communication.
macro_rules! prepare_buffers {
    () => {
        ([0u8; 1536], [0u8; 1536], [0u8; 4096], [0u8; 4096])
    };
}

#[cfg(feature = "mqtt")]
#[macro_export]
/// Macro to wait until WiFi is connected in async variation
/// Typically used after `net_task` async task call.
macro_rules! wait_wifi {
    ($stack:expr, $config:ident) => {
        loop {
            if $stack.is_link_up() {
                break;
            }
            embassy_time::Timer::after(embassy_time::Duration::from_millis(500)).await;
        }
    };
}

/// Macro to retrieve the IP configuration from the network stack.
#[cfg(feature = "mqtt")]
#[macro_export]
macro_rules! get_ip {
    ($stack:expr, $config:ident) => {
        loop {
            if let Some($config) = $stack.config_v4() {
                println!("Got IP: {}", $config.address); // dhcp IP address
                break;
            }
            embassy_time::Timer::after(embassy_time::Duration::from_millis(500)).await;
        }
    };
}

/// Macro to create a network stack for WiFi communication.
#[cfg(feature = "mqtt")]
#[macro_export]
macro_rules! create_stack {
    ($wifi_interface:expr, $config:expr) => {{
        let seed = 1234;

        &*static_cell::make_static!(embassy_net::Stack::new(
            $wifi_interface,
            $config,
            static_cell::make_static!(embassy_net::StackResources::<3>::new()),
            seed
        ))
    }};
}

/// Macro to obtain a suitable timer based on the ESP device mod
#[cfg(not(any(feature = "esp32", feature = "esp32s2", feature = "esp32s3")))]
#[macro_export]
macro_rules! get_timer {
    ($peripherals:ident, $clocks:ident) => {
        esp_hal::systimer::SystemTimer::new($peripherals.SYSTIMER).alarm0
    };
}

#[cfg(any(feature = "esp32", feature = "esp32s2", feature = "esp32s3"))]
#[macro_export]
macro_rules! get_timer {
    ($peripherals:ident, $clocks:ident) => {
        esp_hal::timer::TimerGroup::new($peripherals.TIMG1, &$clocks).timer0
    };
}

/// Macro to initialize the WiFi interface with the given SSID and password in
/// `mqtt` (or async) configuration. This macro configures the WiFi controller
/// and initializes the WiFi interface.
/// Example:
/// ```no_run
/// let peripherals = take_periph!();
/// let system = take_system!(peripherals);
/// let (clocks, pins) = initialize_chip!(peripherals, system);
/// let timer = get_timer(peripherals, clocks);
///
/// let mut socket_set_entries: [smoltcp::iface::SocketStorage; 3] = Default::default();
///
/// embassy::init(&clocks, timer);
///
/// let (mut wifi_interface, controller) =
///     init_wifi!(SSID, PASS, peripherals, system, clocks, socket_set_entries);
/// ```
///
/// # Non-async version of function (`mqtt` feature disabled)
/// Initializes the WiFi interface with the given SSID and password. This macro
/// sets up the WiFi controller, starts the WiFi subsystem, and connects to the
/// specified network. It also initiates a WiFi scan and waits for an IP address
/// to be assigned.
///
/// # Arguments
/// * `$ssid` - The SSID of the WiFi network to connect to.
/// * `$password` - The password of the WiFi network.
/// * `$peripherals` - The ESP peripherals instance, providing access to the
///   device's peripherals.
/// * `$system` - The system peripheral instance, used for system-level
///   configurations.
/// * `$clocks` - The clocks configuration, used for timing and delays.
/// * `$sock_entries` - Mutable reference to the socket entries, used for
///   network socket management.
///
/// # Returns
/// Returns a tuple containing the initialized `WifiStack`, along with two
/// buffers for network operations.
///
/// # Usage
/// This macro is intended to be used for setting up WiFi connectivity in
/// environments where asynchronous operations are NOT used.
///
/// # Example
/// ```no_run
/// let (wifi_stack, rx_buffer, tx_buffer) =
///     init_wifi!(SSID, PASSWORD, peripherals, system, clocks, sock_entries);
/// ```
#[cfg(feature = "mqtt")]
#[macro_export]
macro_rules! init_wifi {
    ($ssid:expr, $password:expr, $peripherals:ident, $system:ident, $clocks:ident) => {{
        let init = esp_wifi::initialize(
            esp_wifi::EspWifiInitFor::Wifi,
            esp_ward::get_timer!($peripherals, $clocks),
            esp_hal::rng::Rng::new($peripherals.RNG),
            $system.radio_clock_control,
            &$clocks,
        )
        .unwrap();

        let wifi = $peripherals.WIFI;
        let (wifi_interface, controller) =
            esp_wifi::wifi::new_with_mode(&init, wifi, esp_wifi::wifi::WifiStaDevice).unwrap();

        (wifi_interface, controller)
    }};
}

#[cfg(all(not(feature = "mqtt"), feature = "wifi"))]
#[macro_export]
macro_rules! init_wifi {
    ($ssid:expr, $password:expr, $peripherals:ident, $system:ident, $clocks:ident, $sock_entries:ident) => {{
        let init = esp_wifi::initialize(
            esp_wifi::EspWifiInitFor::Wifi,
            esp_ward::get_timer!($peripherals, $clocks),
            esp_hal::rng::Rng::new($peripherals.RNG),
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

/// Converts a string IP address into a 4-byte array.
///
/// # Arguments
/// * `ip` - A string slice representing the IP address.
///
/// # Returns
/// A result containing the IP address as a `[u8; 4]` array or an error message
/// if the conversion fails.
#[cfg(feature = "wifi")]
pub fn ip_string_to_parts(ip: &str) -> Result<[u8; 4], &'static str> {
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

/// Extracts a UNIX timestamp from a server response.
///
/// # Arguments
/// * `response` - A byte slice containing the server's response.
///
/// # Returns
/// An option containing the UNIX timestamp if found and successfully parsed, or
/// `None` otherwise.
#[cfg(feature = "wifi")]
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

/// Converts a UNIX timestamp into hours, minutes, and seconds.
///
/// # Arguments
/// * `timestamp` - The UNIX timestamp to convert.
///
/// # Returns
/// A tuple containing the hours, minutes, and seconds.
#[cfg(feature = "wifi")]
pub fn timestamp_to_hms(timestamp: u64) -> (u8, u8, u8) {
    let seconds_per_minute = 60;
    let minutes_per_hour = 60;
    let hours_per_day = 24;
    let seconds_per_hour = seconds_per_minute * minutes_per_hour;
    let seconds_per_day = seconds_per_hour * hours_per_day;

    let hours = (timestamp % seconds_per_day) / seconds_per_hour;
    let minutes = (timestamp % seconds_per_hour) / seconds_per_minute;
    let seconds = timestamp % seconds_per_minute;

    (hours as u8, minutes as u8, seconds as u8)
}

/// Gets a weekday from a UNIX timestamp
///
/// # Arguments
/// * `timestamp` - The UNIX timestamp to convert.
///
/// # Returns
/// String with the name of the day
pub fn weekday_from_timestamp(timestamp: &u64) -> &'static str {
    let days_since_1970 = timestamp / 86400; // seconds in a day
    let day_of_week = (days_since_1970 + 4) % 7; // Adjusting the offset since 1-1-1970 was a Thursday
    match day_of_week {
        0 => "Sunday",
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        _ => "Error",
    }
}

/// Creates a new socket for communication over WiFi.
///
/// # Arguments
/// * `wifi_stack` - Reference to the `WifiStack` to use for creating the
///   socket.
/// * `ip_string` - The IP address as a string to which the socket should
///   connect.
/// * `port` - The port number for the connection.
/// * `rx_buffer` - A mutable reference to the buffer used for receiving data.
/// * `tx_buffer` - A mutable reference to the buffer used for transmitting
///   data.
///
/// # Returns
/// Returns a `Socket` instance ready for communication.
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

/// Sends a request over the specified socket.
///
/// # Arguments
/// * `socket` - A mutable reference to the `Socket` over which to send the
///   request.
/// * `request` - The request string to send.
#[cfg(feature = "wifi")]
pub fn send_request<'a, 's, MODE>(socket: &mut Socket<'s, 'a, MODE>, request: &str)
where
    MODE: WifiDeviceMode,
{
    socket.write(request.as_bytes()).unwrap();
    socket.flush().unwrap();
}

/// Retrieves the current time from the WorldTimeAPI.
///
/// # Arguments
/// * `socket` - The `Socket` to use for making the request to the WorldTimeAPI.
///
/// # Returns
/// Returns a tuple `(u64, u64, u64)` representing the hours, minutes, and
/// seconds if successful. Returns an error otherwise.
#[cfg(feature = "wifi")]
pub fn get_time<'a, 's, MODE>(mut socket: Socket<'s, 'a, MODE>) -> Result<(u8, u8, u8), ()>
where
    MODE: WifiDeviceMode,
{
    let request = "GET /api/timezone/Europe/Prague HTTP/1.1\r\nHost: worldtimeapi.org\r\n\r\n";

    // Using classic "worldtime.api" to get time
    send_request(&mut socket, request);

    let (responce, total_size) = get_responce(socket).unwrap();

    if let Some(timestamp) = find_unixtime(&responce[..total_size]) {
        let mut timestamp = timestamp;
        timestamp += 60 * 60;
        return Ok(timestamp_to_hms(timestamp));
    } else {
        println!("Failed to find or parse the 'unixtime' field.");
        return Err(());
    }
}

/// Retrieves the current time as a UNIX timestamp from the WorldTimeAPI. 
///
/// # Arguments
/// * `socket` - The `Socket` to use for making the request to the WorldTimeAPI.
///
/// # Returns
/// Returns a timestamp representing time if successful. Returns an error
/// otherwise.
#[cfg(feature = "wifi")]
pub fn get_timestamp<'a, 's, MODE>(mut socket: Socket<'s, 'a, MODE>) -> Result<u64, ()>
where
    MODE: WifiDeviceMode,
{
    let request = "GET /api/timezone/Europe/Prague HTTP/1.1\r\nHost: worldtimeapi.org\r\n\r\n";

    // Using classic "worldtime.api" to get time
    send_request(&mut socket, request);

    let (responce, total_size) = get_responce(socket).unwrap();

    if let Some(timestamp) = find_unixtime(&responce[..total_size]) {
        let mut timestamp = timestamp;
        timestamp += 60 * 60;
        return Ok(timestamp);
    } else {
        println!("Failed to find or parse the 'unixtime' field.");
        return Err(());
    }
}

/// Receives a message over the specified socket.
///
/// # Arguments
/// * `socket` - The `Socket` from which to read the message.
///
/// # Returns
/// Returns a tuple containing the message as a byte array and the size of the
/// message if successful. Returns an error otherwise.

#[cfg(feature = "wifi")]
pub fn get_responce<'a, 's, MODE>(
    mut socket: Socket<'s, 'a, MODE>,
) -> Result<([u8; 4096], usize), ()>
where
    MODE: WifiDeviceMode,
{
    let mut buffer = [0u8; 4096];
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

    Ok((buffer, total_size))
}

/// Establishes a default MQTT connection with predefined settings: HiveMQ
/// broker
///
/// # Arguments
/// * `stack` - A reference to the network stack.
/// * `client_id` - The MQTT client identifier.
/// * `rx_buffer_socket` - Receive buffer for the socket.
/// * `tx_buffer_socket` - Transmit buffer for the socket.
/// * `write_buffer_mqtt` - Write buffer for MQTT client.
/// * `recv_buffer_mqtt` - Receive buffer for MQTT client.
///
/// # Returns
/// An `MqttClient` instance configured for communication.
#[cfg(feature = "mqtt")]
pub async fn mqtt_connect_default<'a>(
    stack: &'static Stack<WifiDevice<'static, WifiStaDevice>>,
    client_id: &'a str,
    rx_buffer_socket: &'a mut [u8],
    tx_buffer_socket: &'a mut [u8],
    write_buffer_mqtt: &'a mut [u8; 4096],
    recv_buffer_mqtt: &'a mut [u8; 4096],
) -> MqttClient<'a, TcpSocket<'a>, 5, CountingRng> {
    mqtt_connect_custom(
        stack,
        client_id,
        rx_buffer_socket,
        tx_buffer_socket,
        write_buffer_mqtt,
        recv_buffer_mqtt,
        "mqtt-dashboard.com",
        1883,
        None,
        None,
    )
    .await
}

/// Establishes a custom MQTT connection with the specified parameters.
///
/// # Arguments
/// * `stack` - The network `Stack` to use for the MQTT connection.
/// * `client_id` - The client ID for the MQTT session.
/// * `rx_buffer_socket` - Receive buffer for the socket connection.
/// * `tx_buffer_socket` - Transmit buffer for the socket connection.
/// * `write_buffer_mqtt` - Write buffer for the MQTT client.
/// * `recv_buffer_mqtt` - Receive buffer for the MQTT client.
/// * `broker_address` - The address of the MQTT broker.
/// * `broker_port` - The port of the MQTT broker.
/// * `username` - Optional username for MQTT broker authentication.
/// * `password` - Optional password for MQTT broker authentication.
///
/// # Returns
/// Returns an `MqttClient` instance configured for the specified broker and
/// credentials.
#[cfg(feature = "mqtt")]
pub async fn mqtt_connect_custom<'a>(
    stack: &'static Stack<WifiDevice<'static, WifiStaDevice>>,
    client_id: &'a str,
    rx_buffer_socket: &'a mut [u8],
    tx_buffer_socket: &'a mut [u8],
    write_buffer_mqtt: &'a mut [u8; 4096],
    recv_buffer_mqtt: &'a mut [u8; 4096],
    broker_address: &str, // IP address or hostname of the MQTT broker
    broker_port: u16,     /* Port of the MQTT broker (usually 1883 for MQTT, 8883 for MQTT
                           * over SSL, but make sure to unclude some TSL certification in your
                           * code then) */
    username: Option<&'a str>, // Optional username for MQTT broker authentication
    password: Option<&'a str>, // Optional password for MQTT broker authentication
) -> MqttClient<'a, TcpSocket<'a>, 5, CountingRng> {
    let mut socket = TcpSocket::new(stack, rx_buffer_socket, tx_buffer_socket);

    socket.set_timeout(Some(embassy_time::Duration::from_secs(10)));

    let address = match stack
        .dns_query(broker_address, DnsQueryType::A)
        .await
        .map(|a| a[0])
    {
        Ok(addr) => addr,
        Err(_) => {
            let addr = ip_string_to_parts(broker_address).unwrap();
            IpAddress::v4(addr[0], addr[1], addr[2], addr[3])
        }
    };

    let remote_endpoint = (address, broker_port);
    println!("connecting...");
    let connection = socket.connect(remote_endpoint).await;
    if let Err(e) = connection {
        println!("connect error: {:?}", e);
    }
    println!("connected!");

    let mut config = ClientConfig::new(
        rust_mqtt::client::client_config::MqttVersion::MQTTv5,
        CountingRng(20000),
    );
    config.add_max_subscribe_qos(rust_mqtt::packet::v5::publish_packet::QualityOfService::QoS1);
    config.add_client_id(client_id);
    config.max_packet_size = 149504;

    // Optionally set the username and password
    if let Some(user) = username {
        config.add_username(user);
    }
    if let Some(pass) = password {
        config.add_password(pass);
    }

    let mut client = MqttClient::<_, 5, _>::new(
        socket,
        write_buffer_mqtt,
        4096,
        recv_buffer_mqtt,
        4096,
        config,
    );

    loop {
        match client.connect_to_broker().await {
            Ok(()) => {
                println!("Connected to broker!");
                return client;
            }
            Err(mqtt_error) => match mqtt_error {
                ReasonCode::NetworkError => {
                    println!("MQTT Network Error");
                }
                _ => {
                    println!("Other MQTT Error: {:?}", mqtt_error);
                }
            },
        }
    }
}

/// Runs the network stack for handling MQTT communication.
///
/// # Arguments
/// * `stack` - Reference to the static network stack instance used for MQTT
///   operations.
#[cfg(all(feature = "async", not(feature = "docs")))]
#[embassy_executor::task]
pub async fn net_task(stack: &'static Stack<WifiDevice<'static, WifiStaDevice>>) {
    println!("Start net task");
    stack.run().await;
}

/// Manages WiFi connectivity, ensuring the device is connected to the specified
/// network. This task continuously checks the WiFi connection state and
/// attempts to reconnect if the connection is lost.
///
/// # Arguments
/// * `controller` - The WiFi controller for managing WiFi state and
///   configuration.
/// * `ssid` - The SSID of the WiFi network to connect to.
/// * `pass` - The password for the WiFi network.
#[cfg(all(feature = "async", not(feature = "docs")))]
#[embassy_executor::task]
pub async fn connection(
    mut controller: WifiController<'static>,
    ssid: &'static str,
    pass: &'static str,
) {
    println!("start connection task");
    println!("Device capabilities: {:?}", controller.get_capabilities());
    loop {
        match esp_wifi::wifi::get_wifi_state() {
            WifiState::StaConnected => {
                // wait until we're no longer connected
                controller.wait_for_event(WifiEvent::StaDisconnected).await;
                Timer::after(Duration::from_millis(5000)).await
            }
            _ => {}
        }
        if !matches!(controller.is_started(), Ok(true)) {
            let client_config = Configuration::Client(ClientConfiguration {
                ssid: ssid.try_into().unwrap(),
                password: pass.try_into().unwrap(),
                ..Default::default()
            });
            controller
                .set_configuration(&(&client_config).into())
                .unwrap();
            println!("Starting wifi");
            controller.start().await.unwrap();
            println!("Wifi started!");
        }
        println!("About to connect...");

        match controller.connect().await {
            Ok(_) => println!("Wifi connected!"),
            Err(e) => {
                println!("Failed to connect to wifi: {e:?}");
                Timer::after(Duration::from_millis(5000)).await
            }
        }
    }
}

/// This function attempts to send the message to a specific MQTT topic and
/// retries in case of network errors.
///
/// # Arguments
/// * `client` - A mutable reference to the MQTT client used for sending the
///   message.
/// * `topic_name` - The MQTT topic to which the message will be sent.
/// * `message` - The message payload as a string slice.
#[cfg(feature = "mqtt")]
pub async fn mqtt_send<'a>(
    client: &mut MqttClient<'a, TcpSocket<'a>, 5, CountingRng>,
    topic_name: &'a str,
    message: &'a str,
) {
    loop {
        println!("About to send message");
        match client
            .send_message(
                topic_name,
                message.as_bytes(),
                rust_mqtt::packet::v5::publish_packet::QualityOfService::QoS1,
                true,
            )
            .await
        {
            Ok(()) => {
                println!("Message sent");
                break;
            }
            Err(mqtt_error) => match mqtt_error {
                ReasonCode::NetworkError => {
                    println!("MQTT Network Error");
                    match client.connect_to_broker().await {
                        Ok(()) => {
                            println!("Reconnected to broker!");
                            continue;
                        }
                        Err(mqtt_error) => match mqtt_error {
                            ReasonCode::NetworkError => {
                                println!("MQTT Network Error");
                            }
                            _ => {
                                println!("Other MQTT Error: {:?}", mqtt_error);
                            }
                        },
                    }
                    continue;
                }
                _ => {
                    println!("Other MQTT Error: {:?}", mqtt_error);
                    continue;
                }
            },
        }
    }
}

/// Subscribes to an MQTT topic.
///
/// # Arguments
/// * `client` - A mutable reference to the MQTT client used for the
///   subscription.
/// * `topic_name` - The MQTT topic to which the client will subscribe.
///
/// This function attempts to subscribe to the topic and retries in case of
/// network errors.

#[cfg(feature = "mqtt")]
pub async fn mqtt_subscribe<'a>(
    client: &mut MqttClient<'a, TcpSocket<'a>, 5, CountingRng>,
    topic_name: &'a str,
) {
    loop {
        println!("About to subscribe to topic");
        match client.subscribe_to_topic(topic_name).await {
            Ok(()) => {
                println!("Subscribed to topic");
                break;
            }
            Err(mqtt_error) => match mqtt_error {
                ReasonCode::NetworkError => {
                    println!("MQTT Network Error");
                    match client.connect_to_broker().await {
                        Ok(()) => {
                            println!("Reconnected to broker!");
                            continue;
                        }
                        Err(mqtt_error) => match mqtt_error {
                            ReasonCode::NetworkError => {
                                println!("MQTT Network Error");
                            }
                            _ => {
                                println!("Other MQTT Error: {:?}", mqtt_error);
                            }
                        },
                    }
                    continue;
                }
                _ => {
                    println!("Other MQTT Error: {:?}", mqtt_error);
                    continue;
                }
            },
        }
    }
}

/// Waits for and receives a message from the subscribed MQTT topics.
/// It handles reconnection in case of network errors.
/// # Arguments
/// * `client` - A mutable reference to the MQTT client used for receiving
///   messages.
///
/// # Returns
/// Returns a `String` containing the received message if successful.
#[cfg(feature = "mqtt")]
pub async fn mqtt_receive<'a>(
    client: &mut MqttClient<'a, TcpSocket<'a>, 5, CountingRng>,
) -> String<32> {
    loop {
        match client.receive_message().await {
            Ok((msg_str, _)) => {
                println!("Message received: {}", msg_str);
                let mut string_to_return: String<32> = String::new();
                write!(string_to_return, "{}", msg_str).expect("write! failed...");
                return string_to_return;
            }
            Err(mqtt_error) => match mqtt_error {
                ReasonCode::NetworkError => {
                    match client.connect_to_broker().await {
                        Ok(()) => {
                            println!("Reconnected to broker!");
                            continue;
                        }
                        Err(mqtt_error) => match mqtt_error {
                            ReasonCode::NetworkError => {
                                println!("MQTT Network Error");
                            }
                            _ => {
                                println!("Other MQTT Error: {:?}", mqtt_error);
                            }
                        },
                    }
                    continue;
                }
                _ => {
                    println!("Other MQTT Error or no messages yet");
                    continue;
                }
            },
        }
    }
}
#[cfg(feature = "mqtt")]
pub use create_stack;
#[cfg(feature = "mqtt")]
pub use get_ip;
#[cfg(feature = "wifi")]
pub use get_timer;
#[cfg(feature = "wifi")]
pub use init_wifi;
#[cfg(feature = "mqtt")]
pub use prepare_buffers;
#[cfg(feature = "mqtt")]
pub use wait_wifi;
