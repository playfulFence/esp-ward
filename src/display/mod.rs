pub enum DisplayType {
    Ili9341,
    Max7219,
    Pcd8544,
}

pub mod ili9341;
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