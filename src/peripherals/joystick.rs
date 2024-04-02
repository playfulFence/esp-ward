//! # Joystick Module
//!
//! Provides an interface to a 2-axis joystick with an integrated select button.
//! This module is configured to use specific pins for the X and Y axes and
//! assumes that the select button uses a digital input pin.
//!
//! ### ATTENTION: THIS MODULE IS SUPPOSED TO BE USED ONLY WITH X-AXIS AND Y-AXIS CONNECTED TO DEFAULT PINS!!!
use embedded_hal::{adc::OneShot, digital::v2::InputPin};
use esp_hal::{
    analog::adc::{AdcPin, ADC},
    gpio::{Analog, GpioPin},
    prelude::*,
};

/// Represents a joystick with two axes and a select button.
pub struct Joystick<SELECT: InputPin> {
    /// The select button of the joystick, wrapped in a `Button` struct for
    /// debouncing.
    pub select: crate::peripherals::button::Button<SELECT>,
    /// The analog input pin for the X-axis.
    #[cfg(any(feature = "esp32s2"))]
    pub x_axis: AdcPin<GpioPin<Analog, 1>, esp_hal::peripherals::ADC1>,
    #[cfg(any(feature = "esp32"))]
    pub x_axis: AdcPin<GpioPin<Analog, 35>, esp_hal::peripherals::ADC1>,
    #[cfg(any(feature = "esp32s3"))]
    pub x_axis: AdcPin<GpioPin<Analog, 3>, esp_hal::peripherals::ADC1>,
    #[cfg(any(feature = "esp32h2"))]
    pub x_axis: AdcPin<GpioPin<Analog, 4>, esp_hal::peripherals::ADC1>,
    #[cfg(not(any(feature = "esp32s2", feature = "esp32s3", feature = "esp32", feature = "esp32h2")))]
    pub x_axis: AdcPin<GpioPin<Analog, 0>, esp_hal::peripherals::ADC1>,
    /// The analog input pin for the Y-axis.
    #[cfg(any(feature = "esp32"))]
    pub y_axis: AdcPin<GpioPin<Analog, 36>, esp_hal::peripherals::ADC1>,
    #[cfg(any(feature = "esp32h2"))]
    pub y_axis: AdcPin<GpioPin<Analog, 5>, esp_hal::peripherals::ADC1>,
    #[cfg(not(any(feature = "esp32", feature = "esp32h2")))]
    pub y_axis: AdcPin<GpioPin<Analog, 4>, esp_hal::peripherals::ADC1>,
}

/// A threshold value to interpret the joystick's value in direction.
pub const ROUGH_THRESHOLD: u16 = 2048;

/// Macro for creating a `Joystick` instance.
///
/// Unlike a function, this macro can take ownership of parts of the
/// `Peripherals` without consuming the whole `Peripherals` struct. This allows
/// setting up the ADC configuration for the joystick's analog pins and passing
/// it along for ADC initialization with `peripherals.ADC1` without running into
/// Rust's ownership errors.
///
/// Functions in Rust take ownership or borrow the entire value they are given,
/// which would not allow us to partially consume `Peripherals`. This macro,
/// however, performs the setup inline where it's invoked and thus avoids the
/// mentioned ownership issue.
///
/// # Arguments
/// * `$peripherals` - The `esp-hal` `Peripherals` instance.
/// * `$pins` - The `esp-hal` GPIO pins split from `Peripherals`.
/// * `$pin_select` - The GPIO pin used for the joystick's select button.
///
/// # Usage
/// This macro's name still holds "naming convention" of
/// "create_<device_type>", if sensor/peripheral does not work on top of
/// `I2C/SPI` buses. This macro should be used where you have access to
/// the `Peripherals` and the split pins, and it will return a tuple containing
/// the `Joystick` instance and the initialized ADC.
///
/// ```no_run
/// /// let peripherals = take_periph!();
/// let system = take_system!(peripherals);
/// let (clocks, pins) = initialize_chip!(peripherals, system);
/// let (joystick, adc1) = create_joystick!(peripherals, pins, pin_select);
/// ```

#[macro_export]
macro_rules! create_joystick {
    ($peripherals: expr, $pins: expr, $pin_select: expr ) => {{
        let mut adc1_config = esp_hal::analog::adc::AdcConfig::<esp_hal::peripherals::ADC1>::new();
        let mut select = esp_ward::peripherals::Button::new($pin_select);
        let mut x_axis = adc1_config.enable_pin(
            $pins.gpio0.into_analog(),
            esp_hal::analog::adc::Attenuation::Attenuation11dB,
        );
        let mut y_axis = adc1_config.enable_pin(
            $pins.gpio4.into_analog(),
            esp_hal::analog::adc::Attenuation::Attenuation11dB,
        );

        let mut adc1 = esp_hal::analog::adc::ADC::<esp_hal::peripherals::ADC1>::new(
            $peripherals.ADC1,
            adc1_config,
        );

        (
            Joystick {
                select: select,
                x_axis: x_axis,
                y_axis: y_axis,
            },
            adc1,
        )
    }};
}

pub use create_joystick;

impl<SELECT: InputPin<Error = core::convert::Infallible>> Joystick<SELECT> {
    /// Retrieves the current positions of both axes.
    ///
    /// # Arguments
    /// * `adc` - The ADC instance to read the values from the analog pins.
    ///
    /// # Returns
    /// Returns a tuple `(u16, u16)` where the first element is the X-axis value
    /// and the second is the Y-axis value.
    pub fn get_axes(&mut self, mut adc: ADC<'_, esp_hal::peripherals::ADC1>) -> (u16, u16) {
        (
            nb::block!(adc.read(&mut self.x_axis)).unwrap(),
            nb::block!(adc.read(&mut self.y_axis)).unwrap(),
        )
    }

    /// Retrieves the current position of the X-axis.
    ///
    /// # Arguments
    /// * `adc` - The ADC instance to read the value from the analog pin.
    ///
    /// # Returns
    /// Returns a `u16` representing the X-axis value.
    pub fn get_x(&mut self, adc: ADC<'_, esp_hal::peripherals::ADC1>) -> u16 {
        let (x, _) = self.get_axes(adc);
        x
    }

    /// Retrieves the current position of the Y-axis.
    ///
    /// # Arguments
    /// * `adc` - The ADC instance to read the value from the analog pin.
    ///
    /// # Returns
    /// Returns a `u16` representing the Y-axis value.
    pub fn get_y(&mut self, adc: ADC<'_, esp_hal::peripherals::ADC1>) -> u16 {
        let (_, y) = self.get_axes(adc);
        y
    }

    /// Checks if the select button is currently pressed.
    ///
    /// # Arguments
    /// * `delay` - A delay provider for debouncing the button press.
    ///
    /// # Returns
    /// Returns `true` if the select button is pressed; otherwise `false`.
    pub fn select_pressed(&mut self, mut delay: esp_hal::delay::Delay) -> bool {
        if let crate::peripherals::button::Event::Pressed = self.select.poll(&mut delay) {
            return true;
        } else {
            return false;
        }
    }
}
