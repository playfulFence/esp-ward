extern crate alloc;
use alloc::vec::Vec;

use embedded_hal::digital::v2::OutputPin;
use esp_hal::Delay;
use esp_max7219_nostd::{
    clear_with_state,
    draw_point,
    mappings::SingleDisplayData,
    prepare_display,
    show_moving_text_in_loop,
};
use max7219::{connectors::PinConnector, DecodeMode, MAX7219};

pub struct Max7219Display<DIN: OutputPin, CS: OutputPin, CLK: OutputPin> {
    inner: MAX7219<PinConnector<DIN, CS, CLK>>,
    // This vector will contain actual configurations of each display (which points are lit)
    display_state: Vec<[u8; 8]>,
    actual_active: usize,
    delay: Delay,
}

impl<DIN: OutputPin, CS: OutputPin, CLK: OutputPin> Max7219Display<DIN, CS, CLK> {
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

    fn reset(&mut self) {
        &mut self.inner.power_off();
        &mut self.inner.power_on();
    }

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
