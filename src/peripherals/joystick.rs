/// ATTENTION: THIS MODULE SUPPOSED TO BE USED ONLY WITH X-AXIS CONNECTED TO 0
/// AND Y-AXIS CONNECTED TO 4
use embedded_hal::adc::OneShot;
use embedded_hal::digital::v2::InputPin;
use esp_hal::{
    analog::adc::{AdcConfig, AdcPin, Attenuation, ADC},
    gpio::{Analog, GpioPin},
    prelude::*,
};

pub struct Joystick<SELECT: InputPin> {
    pub select: crate::peripherals::Button<SELECT>,
    pub x_axis: AdcPin<GpioPin<Analog, 0>, esp_hal::peripherals::ADC1>,
    pub y_axis: AdcPin<GpioPin<Analog, 4>, esp_hal::peripherals::ADC1>,
}

pub const ROUGH_THRESHOLD: u16 = 2048;

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
    pub fn get_axes(&mut self, mut adc: ADC<'_, esp_hal::peripherals::ADC1>) -> (u16, u16) {
        (
            nb::block!(adc.read(&mut self.x_axis)).unwrap(),
            nb::block!(adc.read(&mut self.y_axis)).unwrap(),
        )
    }

    pub fn get_x(&mut self, mut adc: ADC<'_, esp_hal::peripherals::ADC1>) -> u16 {
        let (x, _) = self.get_axes(adc);
        x
    }

    pub fn get_y(&mut self, mut adc: ADC<'_, esp_hal::peripherals::ADC1>) -> u16 {
        let (_, y) = self.get_axes(adc);
        y
    }

    pub fn select_pressed(&mut self, mut delay: esp_hal::delay::Delay) -> bool {
        if let crate::peripherals::button::Event::Pressed = self.select.poll(&mut delay) {
            return true;
        } else {
            return false;
        }
    }
}
