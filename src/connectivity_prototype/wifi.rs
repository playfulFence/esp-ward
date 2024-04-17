#[cfg(feature = "wifi")]
use embedded_svc::io::{Read, Write};
#[cfg(feature = "wifi")]
use esp_println::println;
#[cfg(feature = "wifi")]
use esp_wifi::{
    current_millis,
    wifi::{WifiController, WifiDevice, WifiDeviceMode, WifiEvent, WifiStaDevice, WifiState},
    wifi_interface::{Socket, WifiStack},
};
#[cfg(feature = "wifi")]
use smoltcp::wire::{IpAddress, Ipv4Address};

/// Represents the IP address for the WorldTime API server.
pub const WORLDTIMEAPI_IP: &str = "213.188.196.246";

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
#[cfg(feature = "wifi")]
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

#[cfg(feature = "wifi")]
pub use get_timer;
#[cfg(feature = "wifi")]
pub use init_wifi;
