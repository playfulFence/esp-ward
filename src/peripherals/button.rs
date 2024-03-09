use embedded_hal::blocking::delay::DelayMs;
use esp_hal::Delay;

// Debouncing algorythm
pub enum Event {
    Pressed,
    Released,
    Nothing,
}
pub struct Button<T> {
    button: T,
    pressed: bool,
}
impl<T: ::embedded_hal::digital::v2::InputPin<Error = core::convert::Infallible>> Button<T> {
    pub fn new(button: T) -> Self {
        Button {
            button,
            pressed: true,
        }
    }
    pub fn check(&mut self) {
        self.pressed = !self.button.is_low().unwrap();
    }

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
}
