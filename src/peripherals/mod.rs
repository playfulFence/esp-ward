//! # Peripherals Module
//!
//! This module provides abstractions over various peripherals that can be
//! connected to an ESP-based system. It includes traits for common peripheral
//! functionality which is able to be unified.
//!
//! The provided traits allow for a standardized interface for operations like
//! reading and writing data, initializing devices over I2C or SPI buses, and
//! specialized functionalities for different sensor types.
//!
//! ## Usage
//! To use a peripheral, initialize and operate with it using the provided
//! traits.
//!
//! ## Examples
//! ```no_run
//! // Example of creating a temperature sensor on an I2C bus
//! let i2c = esp_ward::init_i2c_default!(peripherals, pins, clocks);
//! let sensor = esp_ward::peripherals::aht20::create_on_i2c(i2c_bus, delay).unwrap();
//! let temperature = sensor.get_temperature().unwrap();
//! ```
//!
//! ## Features
//! - Button input handling.
//! - Support for a range of environmental sensors (temperature, humidity,
//!   pressure, movement).
//! - Distance measurement capabilities.
//! - Light intensity sensing.
//! - Gas sensing for CO2 and VOCs.

// Include sub-modules for different peripherals.
pub mod aht20;
pub mod bme280;
pub mod button;
pub mod joystick;
pub mod pir;
pub mod sgp30;
pub mod tsl2591;
#[cfg(any(not(feature = "esp32"), all(feature = "esp32", feature = "wifi")))]
pub mod ultrasonic_distance;

// Internal use of `esp-hal` components.
use esp_hal::{
    delay::Delay,
    i2c::I2C,
    spi::{master::Spi, FullDuplexMode},
};

/// Represents basic errors that can occur in peripheral operations.
#[derive(Debug)]
pub enum PeripheralError {
    InitializationFailed,
    ReadError,
}

/// Trait for peripherals that communicate over I2C.
/// Implementation should provide a method for creating an instance of the
/// peripheral on the I2C bus.
pub trait I2cPeriph {
    type Returnable;
    fn create_on_i2c(
        bus: I2C<'static, esp_hal::peripherals::I2C0>,
        delay: Delay,
    ) -> Result<Self::Returnable, PeripheralError>;
}

/// Trait for peripherals that communicate over SPI.
/// Implementation should provide a method for creating an instance of the
/// peripheral on the I2C bus.
pub trait SpiPeriph {
    type Returnable;
    fn create_on_spi(
        bus: Spi<'static, esp_hal::peripherals::SPI2, FullDuplexMode>,
        delay: Delay,
    ) -> Result<Self::Returnable, PeripheralError>;
}

/// Trait for peripherals that can be explicitly shut down or deactivated.
pub trait Shutdown {
    fn shutdown(&mut self) -> Result<(), PeripheralError>;
}

/// Trait for peripherals capable of reading data which can not be.
pub trait Readable {
    type Output;
    fn read(&self, delay: Delay) -> Result<Self::Output, PeripheralError>;
}

/// Trait for peripherals capable of writing data.
pub trait Writable {
    type Input;
    fn write(&mut self, data: Self::Input) -> Result<(), PeripheralError>;
}

/// Trait for peripherals that can sense temperature.
pub trait TemperatureSensor {
    // Reads the temperature in degrees Celsius
    fn get_temperature(&mut self) -> Result<f32, PeripheralError>;
}

/// Trait for peripherals that can sense humidity levels.
pub trait HumiditySensor {
    /// Reads the humidity level as a percentage.
    fn get_humidity(&mut self) -> Result<f32, PeripheralError>;
}

/// Trait for peripherals that can sense atmospheric pressure.
pub trait PressureSensor {
    /// Reads the atmospheric pressure in hPa (hectopascals).
    fn get_pressure(&mut self) -> Result<f32, PeripheralError>;
}

/// Trait for peripherals that can measure distance.
pub trait DistanceSensor {
    /// Measures the distance from the sensor to the nearest object.
    fn get_distance(&mut self) -> Result<f32, PeripheralError>;
}

/// Trait for peripherals that can measure CO2 (or CO2 equivalent) levels.
pub trait CO2Sensor {
    /// Measures the CO2 (or CO2eq) concentration in the air.
    fn get_co2(&mut self) -> Result<f32, PeripheralError>;
}

/// Trait for peripherals that can measure Volatile Organic Compounds (VOCs) or
/// Total Volatile Organic Compounds (TVOCs).
pub trait VOCSensor {
    /// Measures the concentration of VOCs in the air.
    fn get_voc(&mut self) -> Result<f32, PeripheralError>;
}

/// Trait for peripherals that can measure luminance.
pub trait LumiSensor {
    /// Measures the ambient light intensity in lux.
    fn get_lux(&mut self) -> Result<f32, PeripheralError>;
}
