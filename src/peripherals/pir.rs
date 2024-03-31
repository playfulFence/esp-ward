//! # Passive Infrared (PIR) Sensor Module
//!
//! Provides an abstraction for interfacing with a PIR motion sensor. PIR
//! sensors are commonly used to detect movement within a certain range.
//!
//! ## Example
//! ```no_run
//! use esp_hal::gpio::GpioExt; // Import traits to split pins
//! use esp_hal::peripherals::Peripherals;
//! use your_crate::peripherals::PIRSensor;
//!
//! let peripherals = Peripherals::take().unwrap();
//! let pins = peripherals.GPIO.split();
//!
//! // Suppose the PIR sensor is connected to GPIO5
//! let pir_pin = pins.gpio5.into_pull_up_input(); // Configure the pin
//! as input with pull-up    /// let mut pir_sensor = PIRSensor::new(pir_pin);
//!
//! // Now you can check for motion
//! if pir_sensor.motion_detected() {
//!     println!("Motion detected!");
//! }
//! ```

use embedded_hal::digital::v2::InputPin;

/// Represents a PIR motion sensor connected to a single digital input pin.
pub struct PIRSensor<PIN: InputPin> {
    /// The digital input pin connected to the PIR sensor.
    inner: PIN,
}

impl<PIN: InputPin<Error = core::convert::Infallible>> PIRSensor<PIN> {
    /// Constructs a new `PIRSensor` with the given input pin.
    ///
    /// # Arguments
    /// * `pin` - The digital input pin connected to the PIR sensor.
    ///  # Returns
    /// A new `PIRSensor` instance.
    pub fn create_on_pins(pin: PIN) -> Self {
        PIRSensor { inner: pin }
    }

    /// Checks if motion has been detected by the sensor.
    ///
    /// # Returns
    /// Returns `true` i
    pub fn motion_detected(&mut self) -> bool {
        self.inner.is_high().unwrap()
    }
}
