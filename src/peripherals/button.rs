//! # Button Module
//!
//! This module provides a simple interface for button handling with debouncing
//! algorythm. It can be used to detect press and release events for a button
//! connected to an input pin.
//! ## Example
/// ```no_run
/// use esp_hal::delay::Delay;
/// use esp_ward::peripherals::button::{Button, Event};
///
/// // Suppose the button is connected to GPIO23
/// let peripherals = take_periph!();
/// let system = take_system!(peripherals);
/// let (clocks, pins) = initialize_chip!(peripherals, system);
/// let mut button = Button::new(pins.gpio23.into_pull_up_input());
/// let mut delay = Delay::new(&clocks);
///
/// loop {
///     // With `match`
///     match button.poll(&mut delay) {
///         Event::Pressed => println!("Button pressed!"),
///         Event::Released => println!("Button released!"),
///         Event::Nothing => (),
///     }
///
///     // Or `if let...`
///     if let crate::peripherals::button::Event::Pressed = self.select.poll(&mut delay) {
///         // your callback if button was pressed
///     } else {
///         // your callback if not
///     }
/// }
/// ```
use embedded_hal::blocking::delay::DelayMs;
use esp_hal::delay::Delay;

/// Represents possible events from a button press.
pub enum Event {
    Pressed,
    Released,
    Nothing,
}
/// A generic button that can report press and release events.
pub struct Button<T> {
    /// The input pin connected to the button.
    button: T,
    /// Tracks the current debounced state of the button.
    pressed: bool,
}

/// Creates a new `Button` instance associated with a specific input pin.
///
/// # Arguments
/// * `button` - The input pin the button is connected to.
///
/// # Returns
/// A new `Button` instance that can be used to detect button events.
impl<T: ::embedded_hal::digital::v2::InputPin<Error = core::convert::Infallible>> Button<T> {
    pub fn create_on_pins(button: T) -> Self {
        Button {
            button,
            pressed: true,
        }
    }
    /// Updates the internal state of the button by reading its current state.
    fn check(&mut self) {
        self.pressed = !self.button.is_low().unwrap();
    }

    /// Polls the button to determine its current state and debounce it.
    ///
    /// This method should be called repeatedly to ensure accurate event
    /// detection.
    ///
    /// # Arguments
    /// * `delay` - A delay provider used for debouncing.
    ///
    /// # Returns
    /// An `Event` indicating the debounced state change of the button.
    pub fn poll(&mut self, delay: &mut Delay) -> Event {
        let pressed_now = !self.button.is_low().unwrap();
        if !self.pressed && pressed_now {
            delay.delay_ms(30 as u32);
            self.check();
            if !self.button.is_low().unwrap() {
                Event::Pressed
            } else {
                Event::Nothing
            }
        } else if self.pressed && !pressed_now {
            delay.delay_ms(30 as u32);
            self.check();
            if self.button.is_low().unwrap() {
                Event::Released
            } else {
                Event::Nothing
            }
        } else {
            Event::Nothing
        }
    }

    pub fn pressed(&mut self, mut delay: esp_hal::delay::Delay) -> bool {
        if let crate::peripherals::button::Event::Pressed = self.poll(&mut delay) {
            return true;
        } else {
            return false;
        }
    }
}
