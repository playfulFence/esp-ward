//! # Ultrasonic Distance Sensor Module
//!
//! This module provides support for ultrasonic distance sensors, allowing for
//! the measurement of distances by emitting ultrasonic pulses and measuring the
//! time taken for the echo to return.

const SOUND_SPEED: f32 = 331.3; // Base speed of sound in air at 0 degrees Celsius in m/s
const SOUND_SPEED_INC_OVER_TEMP: f32 = 0.606; // Increase in the speed of sound per degree Celsius

use embedded_hal::{
    blocking::delay::DelayUs,
    digital::v2::{InputPin, OutputPin},
};
use esp_hal::delay::Delay;
#[cfg(not(feature = "esp32"))]
use esp_hal::systimer::SystemTimer;
#[cfg(feature = "esp32")]
use esp_wifi::current_millis;

/// Represents an ultrasonic distance sensor with trigger and echo pins
pub struct USDistanceSensor<TriggerPin, EchoPin>
where
    TriggerPin: OutputPin<Error = core::convert::Infallible>,
    EchoPin: InputPin<Error = core::convert::Infallible>,
{
    trigger: TriggerPin,
    echo: EchoPin,
    delay: Delay,
}

impl<TriggerPin, EchoPin> USDistanceSensor<TriggerPin, EchoPin>
where
    TriggerPin: OutputPin<Error = core::convert::Infallible>,
    EchoPin: InputPin<Error = core::convert::Infallible>,
{
    /// Initializes a new ultrasonic distance sensor.
    ///
    /// # Arguments
    /// * `trigger` - The output pin used to trigger the sensor.
    /// * `echo` - The input pin used to read the echo signal.
    /// * `delay` - Delay provider for timing the trigger pulse.
    ///
    /// # Returns
    /// A new instance of `USDistanceSensor`.
    pub fn create_on_pins(mut trigger: TriggerPin, echo: EchoPin, delay: Delay) -> Self {
        trigger.set_low().unwrap();
        USDistanceSensor {
            trigger,
            echo,
            delay,
        }
    }

    /// Measures the distance to an object by sending an ultrasonic pulse and
    /// measuring the time taken for the echo to return.
    ///
    /// # Arguments
    /// * `ambient_temp` - The ambient temperature in degrees Celsius, used to
    ///   adjust the speed of sound.
    ///
    /// # Returns
    /// The measured distance in meters.
    pub fn get_distance(&mut self, ambient_temp: f32) -> f32 {
        let sound_speed = SOUND_SPEED + (SOUND_SPEED_INC_OVER_TEMP * ambient_temp);
        self.trigger.set_high().unwrap();
        self.delay.delay_us(10 as u32);
        self.trigger.set_low().unwrap();

        while self.echo.is_low().unwrap() {}
        #[cfg(not(feature = "esp32"))]
        let start_timestamp = SystemTimer::now();
        #[cfg(feature = "esp32")]
        let start_timestamp = current_millis();
        while self.echo.is_high().unwrap() {}
        #[cfg(not(feature = "esp32"))]
        let end_timestamp = SystemTimer::now();
        #[cfg(feature = "esp32")]
        let end_timestamp = current_millis();

        sound_speed * ((end_timestamp as f32 - start_timestamp as f32) / 10000.0) / 2.0
    }
}
