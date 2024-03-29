pub enum DisplaySegment {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}
use embedded_graphics::{mono_font::MonoTextStyle, pixelcolor::Rgb565};

pub mod ili9341;
#[cfg(alloc)]
pub mod max7219;
pub mod pcd8544;

pub trait Display {
    // Set a pixel on the display
    fn set_pixel(&mut self, x: usize, y: usize);

    // Write a string on the display (without newline)
    fn write_str(&mut self, str: &str);

    // Refresh the display to show any changes made
    fn reset(&mut self);
}

pub trait EGDisplay {
    fn write_string_to_segment(
        &mut self,
        segment: DisplaySegment,
        text: &str,
        font: MonoTextStyle<Rgb565>,
    );
}
