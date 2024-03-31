//! # MAX7219 Display Driver
//!
//! This module provides a driver for the MAX7219 LED matrix display controller.
//! It allows for operations such as setting individual LEDs on the matrix and
//! displaying scrolling text.
//!
//! This module and usage of `max7219` display requires an allocator, so in
//! order to use this module PLEASE enable the `alloc` feature of `esp-ward` and
//! make sure to call `esp_ward::prepare_alloc!()` in you program!

extern crate alloc;
use alloc::vec::Vec;

use embedded_hal::digital::v2::OutputPin;
use esp_hal::delay::Delay;
use esp_max7219_nostd::{
    clear_with_state,
    draw_point,
    mappings::SingleDisplayData,
    prepare_display,
    show_moving_text_in_loop,
};
use max7219::{connectors::PinConnector, DecodeMode, MAX7219};

/// Represents a MAX7219 display and provides methods to interact with it.
pub struct Max7219Display<DIN: OutputPin, CS: OutputPin, CLK: OutputPin> {
    /// The underlying MAX7219 driver instance.
    inner: MAX7219<PinConnector<DIN, CS, CLK>>,
    /// Current state of the display, tracking which LEDs are lit.
    display_state: Vec<[u8; 8]>,
    /// Index of the currently active display in a chained setup.
    actual_active: usize,
    /// Delay provider for timing-sensitive operations.
    delay: Delay,
}

impl<DIN: OutputPin, CS: OutputPin, CLK: OutputPin> Max7219Display<DIN, CS, CLK> {
    /// Creates and initializes a new `Max7219Display`.
    ///
    /// # Arguments
    /// * `pin_data` - Data input pin connected to the MAX7219.
    /// * `pin_cs` - Chip select pin connected to the MAX7219.
    /// * `pin_clk` - Clock pin connected to the MAX7219.
    /// * `number_of_displays` - The number of daisy-chained MAX7219 units.
    /// * `delay` - Delay provider for timing-sensitive operations.
    ///
    /// # Returns
    /// A `Max7219Display` instance ready to be used.
    pub fn create_display(
        pin_data: DIN,
        pin_cs: CS,
        pin_clk: CLK,
        number_of_displays: usize,
        delay: Delay,
    ) -> Max7219Display<DIN, CS, CLK> {
        let mut display =
            MAX7219::from_pins(number_of_displays, pin_data, pin_cs, pin_clk).unwrap();
        prepare_display(&mut display, number_of_displays, 0x5);

        let mut to_return = Max7219Display {
            inner: display,
            display_state: Vec::new(),
            actual_active: 0,
            delay: delay,
        };

        let mut tmp = [0b00000000 as u8; 8];

        for i in 0..number_of_displays {
            to_return.display_state.push(tmp);
            tmp = [0b00000000 as u8; 8];
        }

        to_return
    }
}

impl<DIN: OutputPin, CS: OutputPin, CLK: OutputPin> super::Display
    for Max7219Display<DIN, CS, CLK>
{
    /// Sets a pixel on the LED matrix display at the specified coordinates.
    ///
    /// # Arguments
    /// * `x` - The x coordinate on the display matrix.
    /// * `y` - The y coordinate on the display matrix.

    fn set_pixel(&mut self, x: usize, y: usize) {
        if y > 8 || x > 8 * self.display_state.len() {
            panic!("passed coordinates are not available in you Max7219 display configuration");
        }
        // Determine which display in the chain
        let display_index = x / 8;

        // Determine the x coordinate within the targeted display
        let local_x = x % 8;

        draw_point(
            &mut self.inner,
            display_index,
            &mut self.display_state[display_index],
            local_x,
            y,
        );
    }

    /// Resets the display, turning all LEDs off and then back on.
    ///
    /// This can be used to clear any residual data from the display's memory.
    fn reset(&mut self) {
        &mut self.inner.power_off();
        &mut self.inner.power_on();
    }

    /// Displays scrolling text across the LED matrix display.
    ///
    /// # Arguments
    /// * `str` - The string of text to display.
    ///
    /// Scrolls the text horizontally across the display, wrapping around as
    /// needed.
    fn write_str(&mut self, str: &str) {
        show_moving_text_in_loop(
            &mut self.inner,
            str,
            self.display_state.len(),
            25,
            2,
            &mut self.delay,
        )
    }
}
