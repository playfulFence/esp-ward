//! # PCD8544 Display Driver
//!
//! This module provides a driver for the PCD8544 LCD display, commonly known as
//! the Nokia 5110/3310 screen. It provides a simple interface for initializing
//! the display and drawing text and pixels.

use core::fmt::Write;

use embedded_hal::digital::v2::OutputPin;
use pcd8544::PCD8544;

/// Represents a PCD8544 display and provides methods to interact with it.
pub struct Pcd8544Display<
    CLK: OutputPin,
    DIN: OutputPin,
    DC: OutputPin,
    CE: OutputPin,
    RST: OutputPin,
    BL: OutputPin,
> {
    /// The underlying PCD8544 driver instance.
    pub inner: PCD8544<CLK, DIN, DC, CE, RST, BL>,
}

impl<
        CLK: OutputPin<Error = core::convert::Infallible>,
        DIN: OutputPin<Error = core::convert::Infallible>,
        DC: OutputPin<Error = core::convert::Infallible>,
        CE: OutputPin<Error = core::convert::Infallible>,
        RST: OutputPin<Error = core::convert::Infallible>,
        BL: OutputPin<Error = core::convert::Infallible>,
    > Pcd8544Display<CLK, DIN, DC, CE, RST, BL>
{
    /// Creates and initializes a new `Pcd8544Display`.
    ///
    /// # Arguments
    /// * `pin_clk` - Clock pin.
    /// * `pin_data` - Data input pin.
    /// * `pin_dc` - Data/command mode select pin.
    /// * `pin_ce` - Chip enable pin.
    /// * `pin_rst` - Reset pin.
    /// * `pin_backlight` - Backlight control pin.
    ///
    /// # Returns
    /// A `Pcd8544Display` instance ready to be used.
    pub fn create_display(
        pin_clk: CLK,
        pin_data: DIN,
        pin_dc: DC,
        pin_ce: CE,
        pin_rst: RST,
        pin_backlight: BL,
    ) -> Pcd8544Display<CLK, DIN, DC, CE, RST, BL> {
        let mut display =
            PCD8544::new(pin_clk, pin_data, pin_dc, pin_ce, pin_rst, pin_backlight).unwrap();
        display.reset().unwrap();
        display.set_light(true).unwrap();
        Pcd8544Display { inner: display }
    }
}

impl<
        CLK: OutputPin<Error = core::convert::Infallible>,
        DIN: OutputPin<Error = core::convert::Infallible>,
        DC: OutputPin<Error = core::convert::Infallible>,
        CE: OutputPin<Error = core::convert::Infallible>,
        RST: OutputPin<Error = core::convert::Infallible>,
        BL: OutputPin<Error = core::convert::Infallible>,
    > super::Display for Pcd8544Display<CLK, DIN, DC, CE, RST, BL>
{
    /// Sets a pixel on the display at the specified coordinates.
    ///
    /// # Arguments
    /// * `x` - The x coordinate of the pixel.
    /// * `y` - The y coordinate of the pixel.
    fn set_pixel(&mut self, x: usize, y: usize) {
        self.inner
            .set_pixel(x.try_into().unwrap(), y.try_into().unwrap())
            .unwrap();
    }
    /// Resets the display
    fn reset(&mut self) {
        self.inner.reset().unwrap();
    }
    /// Writes a string to the display.
    ///
    /// # Arguments
    /// * `str` - The string to be written on the display.
    fn write_str(&mut self, str: &str) {
        self.inner.write_str(str).unwrap();
    }
}
