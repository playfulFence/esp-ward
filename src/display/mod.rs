//! # Display Module
//!
//! This module provides abstractions over different types of displays, offering
//! a set of traits for basic display operations and integrating with the
//! `embedded_graphics` library for more advanced features.
//!
//! The `Display` trait offers fundamental operations like setting pixels and
//! writing strings, while the `EGDisplay` trait is tailored for displays that
//! work with the `embedded_graphics` library, enabling the use of fonts and
//! more complex drawing operations.
//! Also there is no common peripheral initializing function in trait since all
//! displays require different parameters for it, but constructor is available
//! for every display via `create_display`
//!
//! ## Currently Supported Displays
//! - `ili9341`: A driver for the ILI9341 LCD display.
//! - `max7219`: A driver for the MAX7219 LED display matrix (requires the
//!   `alloc` feature).
//! - `pcd8544`: A driver for the PCD8544 LCD display used in Nokia 5110/3310.

/// Represents segments of a display which can be targeted for writing text or
/// graphics
pub enum DisplaySegment {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

// External imports from the `embedded_graphics` crate.
use embedded_graphics::{mono_font::MonoTextStyle, pixelcolor::Rgb565};

// Include sub-modules for different display drivers.
pub mod ili9341;
#[cfg(alloc)]
pub mod max7219;
pub mod pcd8544;

/// Provides a basic set of operations for interacting with a display.
pub trait Display {
    /// Sets a single pixel on the display to a specified coordinates.
    fn set_pixel(&mut self, x: usize, y: usize);

    /// Writes a string to the display at the current cursor position without a
    /// newline.
    fn write_str(&mut self, str: &str);

    /// Resets the display.
    fn reset(&mut self);
}

/// Extension of the `Display` trait to integrate with the `embedded_graphics`
/// library.
pub trait EGDisplay {
    /// Writes a string to a specific segment of the display using a specified
    /// font style.
    fn write_string_to_segment(
        &mut self,
        segment: DisplaySegment,
        text: &str,
        font: MonoTextStyle<Rgb565>,
    );

    /// Writes a section name to a specific segment of the display using a
    /// specified font style. This can be used to label parts of the display
    /// for better organization and readability.
    fn write_section_name(
        &mut self,
        segment: DisplaySegment,
        name: &str,
        font: MonoTextStyle<Rgb565>,
    );
}
