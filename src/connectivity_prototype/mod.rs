//! # Connectivity Features
//!
//! This module provides functionality for initializing and managing WiFi
//! connections on ESP devices, including MQTT messaging

#[cfg(feature = "mqtt")]
pub mod mqtt;
pub mod wifi;
// TO BE FIXED (Blocked by "esp-wifi")
// pub mod tiny_mqtt;

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
