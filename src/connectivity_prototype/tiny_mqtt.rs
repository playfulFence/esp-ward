//! # Tiny MQTT Client
//!
//! This module attempts to provide a lightweight MQTT client for ESP platforms
//! without using async. It's based on the existing driver from one of the
//! `esp-rs` team members but is not fully functional due to breaking changes in `esp-wifi` (see [issue](https://github.com/esp-rs/esp-wifi/issues/446))
//! This implementation will be completed once the necessary changes in
//! `esp-wifi` are made.

use embedded_io::{Read, Write};
use esp_println::println;
use esp_wifi::{
    compat::queue::SimpleQueue,
    wifi::{WifiDeviceMode, WifiError},
};
use mqttrust::{
    encoding::v4::{decode_slice, encode_slice, Connect, Pid, Protocol},
    Mqtt,
    MqttError,
    Packet,
    Publish,
    QoS,
    Subscribe,
    SubscribeTopic,
};
use smoltcp::wire::IpAddress;
use static_cell::make_static;

/// Represents errors that can occur in the Tiny MQTT client.
#[derive(Debug)]
pub enum TinyMqttError {
    MqttError(MqttError),
    WifiError(WifiError),
}

impl From<MqttError> for TinyMqttError {
    fn from(e: MqttError) -> Self {
        TinyMqttError::MqttError(e)
    }
}

impl From<WifiError> for TinyMqttError {
    fn from(e: WifiError) -> Self {
        TinyMqttError::WifiError(e)
    }
}

#[derive(Copy, Clone)]
pub struct PacketBuffer {
    bytes: [u8; 1024],
}

impl PacketBuffer {
    /// Creates a new `PacketBuffer` from a given MQTT packet.
    ///
    /// # Arguments
    /// * `packet` - The MQTT packet to encode into the buffer.
    pub fn new(packet: Packet<'_>) -> PacketBuffer {
        let mut buf = [0u8; 1024];
        encode_slice(&packet, &mut buf).ok();
        PacketBuffer { bytes: buf }
    }

    /// Parses the stored bytes back into an MQTT packet.
    ///
    /// # Returns
    /// Returns the parsed MQTT packet.
    pub fn parsed(&self) -> Packet<'_> {
        // this might panic: "InvalidPid(0)" when I send s.th with QoS > 0
        decode_slice(&self.bytes).unwrap().unwrap()
    }
}

/// The main structure representing the Tiny MQTT client.
pub struct TinyMqtt<'s, 'a, MODE: WifiDeviceMode> {
    client_id: &'a str,
    socket: esp_wifi::wifi_interface::Socket<'s, 'a, MODE>,
    queue: core::cell::RefCell<SimpleQueue<(usize, [u8; 1024]), 10>>,
    recv_buffer: [u8; 1024],
    recv_index: usize,
    recv_queue: core::cell::RefCell<SimpleQueue<PacketBuffer, 10>>,
    timeout_secs: u16,
    last_sent_millis: u64,
    current_millis_fn: fn() -> u64,
    receive_callback: Option<&'a dyn Fn(&str, &[u8])>,
}

impl<'a, 's, MODE: WifiDeviceMode> TinyMqtt<'s, 'a, MODE> {
    /// Creates a new instance of `TinyMqtt`.
    ///
    /// # Arguments
    /// * `client_id` - The client ID to use for MQTT sessions.
    /// * `socket` - The WiFi socket to use for communication.
    /// * `current_millis_fn` - A function that returns the current time in
    ///   milliseconds.
    /// * `receive_callback` - An optional callback for handling received
    ///   messages.
    pub fn new(
        client_id: &'a str,
        socket: esp_wifi::wifi_interface::Socket<'s, 'a, MODE>,
        current_millis_fn: fn() -> u64,
        receive_callback: Option<&'a dyn Fn(&str, &[u8])>,
    ) -> TinyMqtt<'a, 's, MODE>
    where
        MODE: WifiDeviceMode,
    {
        let res = TinyMqtt {
            client_id,
            socket,
            queue: core::cell::RefCell::new(SimpleQueue::new()),
            recv_buffer: [0u8; 1024],
            recv_index: 0,
            recv_queue: core::cell::RefCell::new(SimpleQueue::new()),
            timeout_secs: 0,
            last_sent_millis: 0,
            current_millis_fn,
            receive_callback,
        };

        res
    }

    /// Connects to an MQTT broker.
    ///
    /// # Arguments
    /// * `addr` - The IP address of the MQTT broker.
    /// * `port` - The port of the MQTT broker.
    /// * `keep_alive_secs` - The keep-alive interval in seconds.
    /// * `username` - Optional username for broker authentication.
    /// * `password` - Optional password for broker authentication.
    ///
    /// # Returns
    /// Returns `Ok(())` if the connection was successful, or a `TinyMqttError`
    /// otherwise.
    pub fn connect(
        &mut self,
        addr: IpAddress,
        port: u16,
        keep_alive_secs: u16,
        username: Option<&'a str>,
        password: Option<&'a [u8]>,
    ) -> Result<(), TinyMqttError> {
        self.timeout_secs = keep_alive_secs;

        self.socket.open(addr, port).unwrap();

        let connect = Packet::Connect(Connect {
            protocol: Protocol::MQTT311,
            keep_alive: keep_alive_secs,
            client_id: "", // self.client_id(),
            clean_session: true,
            last_will: None,
            username,
            password,
        });
        self.send(connect)?;
        self.last_sent_millis = (self.current_millis_fn)();

        Ok(())
    }

    /// Disconnects from the MQTT broker.
    ///
    /// # Returns
    /// Returns `Ok(())` if the disconnection was successful, or a
    /// `TinyMqttError` otherwise.
    pub fn disconnect(&mut self) -> Result<(), TinyMqttError> {
        self.socket.disconnect();
        Ok(())
    }

    /// Publishes a message to a specific topic with an optional packet
    /// identifier (PID).
    ///
    /// # Arguments
    /// * `pid` - An optional packet identifier. Required for QoS levels 1 and
    ///   2.
    /// * `topic_name` - The topic name to which the message will be published.
    /// * `payload` - The message payload as a byte slice.
    /// * `qos` - The quality of service level for the message.
    ///
    /// # Returns
    /// Returns `Ok(())` if the message was enqueued for sending successfully,
    /// or an `MqttError` otherwise.
    pub fn publish_with_pid(
        &self,
        pid: Option<Pid>,
        topic_name: &str,
        payload: &[u8],
        qos: QoS,
    ) -> Result<(), MqttError> {
        let packet = Packet::Publish(Publish {
            dup: false,
            qos,
            pid: None,
            retain: false,
            topic_name,
            payload,
        });

        let mut buf = [0u8; 1024];
        let len = encode_slice(&packet, &mut buf).unwrap();

        // encode_slice doesn't fill in the PID for publish packets
        if pid.is_some() {
            let pid: u16 = pid.unwrap().into();
            let idx = len - payload.len() - 2;
            buf[idx + 0] = ((pid & 0xff00) >> 8) as u8;
            buf[idx + 1] = (pid & 0xff) as u8;
        }

        self.queue.borrow_mut().enqueue((len, buf)).unwrap();
        Ok(())
    }

    /// Subscribes to one or more topics.
    ///
    /// # Arguments
    /// * `_pid` - An optional packet identifier. Currently unused.
    /// * `topics` - A slice of `SubscribeTopic` specifying the topics to
    ///   subscribe to.
    ///
    /// # Returns
    /// Returns `Ok(())` if the subscription request was sent successfully, or
    /// an `MqttError` otherwise.
    #[allow(dead_code)]
    pub fn subscribe<'b: 'a>(
        &self,
        _pid: Option<Pid>,
        topics: &[SubscribeTopic<'_>],
    ) -> Result<(), MqttError> {
        let subscribe = Subscribe::new(topics);
        let packet = Packet::Subscribe(subscribe);

        self.send(packet)?;

        Ok(())
    }

    /// Polls for MQTT operations, including sending ping requests, processing
    /// incoming packets, and sending queued messages.
    ///
    /// # Returns
    /// Returns `Ok(())` if polling operations were successful, or a
    /// `TinyMqttError` otherwise.
    pub fn poll(&mut self) -> Result<(), TinyMqttError> {
        self.poll_internal(true)
    }

    /// Internal function for polling operations. Handles sending ping requests,
    /// receiving packets, and sending queued messages.
    ///
    /// # Arguments
    /// * `drain_receive_queue` - Whether to drain the receive queue and process
    ///   incoming packets.
    ///
    /// # Returns
    /// Returns `Ok(())` if internal polling operations were successful, or a
    /// `TinyMqttError` otherwise.
    fn poll_internal(&mut self, drain_receive_queue: bool) -> Result<(), TinyMqttError> {
        let time = (self.current_millis_fn)();
        println!("Inside poll_internal(1)");

        if time > self.last_sent_millis + ((self.timeout_secs as u64 / 2) * 1000) {
            // ping
            self.send(Packet::Pingreq)?;
            self.last_sent_millis = (self.current_millis_fn)();
        }

        println!("Inside poll_internal(2)");

        self.receive_internal()?;
        println!("Inside poll_internal(3)");
        self.send_internal()?;

        println!("Inside poll_internal(4)");

        // just drain the received packets for now
        if drain_receive_queue {
            while let Some(received) = self.recv_queue.borrow_mut().dequeue() {
                if let Packet::Publish(publish) = received.parsed() {
                    if let Some(callback) = self.receive_callback {
                        callback(publish.topic_name, publish.payload);
                    }
                }
            }
        }

        Ok(())
    }

    /// Internal function to receive MQTT packets and enqueue them for
    /// processing.
    ///
    /// # Returns
    /// Returns `Ok(())` if packets were received and enqueued successfully, or
    /// a `TinyMqttError` otherwise.
    fn receive_internal(&mut self) -> Result<(), TinyMqttError> {
        loop {
            println!("Inside recieve_internal(1)");
            let mut buffer = [0u8; 1024];
            if let Ok(len) = self.socket.read(&mut buffer) {
                println!("Inside recieve_internal(2)");
                if len > 0 {
                    println!("got {} bytes: {:02x?}", len, &buffer[..len]);
                }
                println!("Inside recieve_internal(3)");

                self.recv_buffer[self.recv_index..][..len].copy_from_slice(&buffer[..len]);

                self.recv_index += len;
                println!("Inside recieve_internal(4)");

                let data = self.recv_buffer[..len].as_ref();
                println!("Inside recieve_internal(5)");
                let packet = decode_slice(data);
                println!("Inside recieve_internal(6)");

                if let Ok(Some(packet)) = packet {
                    println!("{:?}", packet);
                    self.recv_index = 0;
                    self.recv_queue
                        .borrow_mut()
                        .enqueue(PacketBuffer::new(packet))
                        .ok();
                }
                println!("Inside recieve_internal(7)");

                if len == 0 {
                    return Ok(());
                }
            } else {
                panic!("Can't read socket");
            }
        }
    }

    /// Internal function to send queued MQTT messages.
    ///
    /// # Returns
    /// Returns `Ok(())` if all queued messages were sent successfully, or a
    /// `TinyMqttError` otherwise.
    fn send_internal(&mut self) -> Result<(), TinyMqttError> {
        loop {
            let dq = self.queue.borrow_mut().dequeue();
            match dq {
                Some((len, buffer)) => loop {
                    println!("try sending a buffer, len = {}", len);
                    if self.socket.write(&buffer[..len]).is_ok() {
                        println!("fine");
                        return Ok(());
                    }
                },
                None => return Ok(()),
            }
        }
    }
}

impl<'a, 's, MODE: WifiDeviceMode> Mqtt for TinyMqtt<'a, 's, MODE> {
    /// Sends an MQTT packet.
    ///
    /// # Arguments
    /// * `packet` - The MQTT packet to send.
    ///
    /// # Returns
    /// Returns `Ok(())` if the packet was enqueued for sending successfully, or
    /// an `MqttError` otherwise.
    fn send(&self, packet: mqttrust::Packet<'_>) -> Result<(), mqttrust::MqttError> {
        let mut buf = [0u8; 1024];
        let len = encode_slice(&packet, &mut buf).unwrap();

        self.queue.borrow_mut().enqueue((len, buf)).ok();
        Ok(())
    }

    /// Retrieves the client ID.
    ///
    /// # Returns
    /// Returns the client ID as a string slice.
    fn client_id(&self) -> &str {
        self.client_id
    }
}

/// Pauses execution for a specified duration in milliseconds.
///
/// # Arguments
/// * `delay` - The number of milliseconds to pause execution.
pub fn sleep_millis(delay: u32) {
    let sleep_end = esp_wifi::current_millis() + delay as u64;
    while esp_wifi::current_millis() < sleep_end {
        // wait
    }
}
