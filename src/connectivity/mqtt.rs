//! this module was written based on a basis of an analysis of an existing
//! project from an esp-rs team member Juraj Sadel, that was shown at Espressif
//! DevCon 2023.
//! Available on: https://github.com/JurajSadel/esp32c3-no-std-async-mqtt-demo

use core::fmt::Write as coreWrite;

use embassy_net::{dns::DnsQueryType, tcp::TcpSocket, Stack};
use embassy_time::{Duration, Timer};
use embedded_svc::wifi::{ClientConfiguration, Configuration};
use esp_println::println;
use esp_wifi::wifi::{WifiController, WifiDevice, WifiEvent, WifiStaDevice, WifiState};
use heapless::String;
use rust_mqtt::{
    client::{client::MqttClient, client_config::ClientConfig},
    packet::v5::reason_codes::ReasonCode,
    utils::rng_generator::CountingRng,
};
use smoltcp::wire::IpAddress;

/// Represents the IP address for the HiveMQ MQTT broker.
pub const HIVE_MQ_IP: &str = "18.196.194.55";
/// Represents the port number for the HiveMQ MQTT broker.
pub const HIVE_MQ_PORT: u16 = 8884;

#[macro_export]
/// Macro to prepare buffers with fixed sizes for MQTT communication.
macro_rules! prepare_buffers {
    () => {
        ([0u8; 1536], [0u8; 1536], [0u8; 4096], [0u8; 4096])
    };
}

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
            let addr = super::wifi::ip_string_to_parts(broker_address).unwrap();
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

pub use create_stack;
pub use get_ip;
pub use prepare_buffers;
pub use wait_wifi;
