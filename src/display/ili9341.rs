//! # ILI9341 Display Driver
//!
//! This module provides a driver for the ILI9341 LCD display using the SPI
//! interface. It includes functionalities to interact with the display at a low
//! level, such as setting pixels and writing strings, as well as high-level
//! operations via the embedded-graphics library.

use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::*,
    mono_font::{MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::*,
    prelude::*,
    primitives::*,
    text::*,
};
use embedded_hal::digital::v2::OutputPin;
use esp_hal::{
    delay::Delay,
    spi,
    spi::{master::prelude::_esp_hal_spi_master_Instance, IsFullDuplex},
};
use profont::{PROFONT_14_POINT, PROFONT_18_POINT, PROFONT_24_POINT};

use super::DisplaySegment;

// Definition of default styles using the ProFont monospace font at different
// sizes.
pub const DEFAULT_STYLE_SMALL: MonoTextStyle<Rgb565> = MonoTextStyleBuilder::new()
    .font(&PROFONT_14_POINT)
    .text_color(RgbColor::BLACK)
    .build();

pub const DEFAULT_STYLE_MID: MonoTextStyle<Rgb565> = MonoTextStyleBuilder::new()
    .font(&PROFONT_18_POINT)
    .text_color(RgbColor::BLACK)
    .build();

pub const DEFAULT_STYLE_LARGE: MonoTextStyle<Rgb565> = MonoTextStyleBuilder::new()
    .font(&PROFONT_24_POINT)
    .text_color(RgbColor::BLACK)
    .build();

/// The `Ili9341Display` struct represents an ILI9341 display connected via SPI.
///
/// It encapsulates the lower-level details of communicating with the display
/// and provides a high-level interface for drawing and text rendering.
pub struct Ili9341Display<
    T: _esp_hal_spi_master_Instance + 'static,
    M: IsFullDuplex,
    RST: OutputPin<Error = core::convert::Infallible>,
    DC: OutputPin<Error = core::convert::Infallible>,
> {
    /// The inner display instance from the `mipidsi` crate configured for
    /// ILI9341 and RGB565 color mode.
    pub inner: mipidsi::Display<
        SPIInterfaceNoCS<spi::master::Spi<'static, T, M>, DC>,
        mipidsi::models::ILI9341Rgb565,
        RST,
    >,
}

impl<
        T: _esp_hal_spi_master_Instance + 'static,
        M: IsFullDuplex,
        RST: OutputPin<Error = core::convert::Infallible>,
        DC: OutputPin<Error = core::convert::Infallible>,
    > Ili9341Display<T, M, RST, DC>
{
    /// Constructs a new `Ili9341Display`.
    ///
    /// Initializes the display over SPI, resets it, and prepares it for drawing
    /// operations.
    /// DOESN'T implement `Display``
    ///
    /// # Arguments
    /// * `spi` - The SPI interface used to communicate with the display.
    /// * `reset` - The pin used to reset the display.
    /// * `dc` - The data/command control pin.
    /// * `delay` - The delay provider to use for timing-sensitive operations.
    ///
    /// # Returns
    /// An initialized `Ili9341Display` object ready for use.
    pub fn create_display(
        spi: spi::master::Spi<'static, T, M>,
        reset: RST,
        dc: DC,
        mut delay: Delay,
    ) -> Ili9341Display<T, M, RST, DC> {
        let di = SPIInterfaceNoCS::new(spi, dc);

        let mut display = mipidsi::Builder::ili9341_rgb565(di)
            .with_display_size(240 as u16, 320 as u16)
            .with_orientation(mipidsi::Orientation::Landscape(true))
            .with_color_order(mipidsi::ColorOrder::Rgb)
            .init(&mut delay, Some(reset))
            .unwrap();

        display.clear(Rgb565::WHITE).unwrap();

        Ili9341Display { inner: display }
    }
}

impl<
        T: _esp_hal_spi_master_Instance + 'static,
        M: IsFullDuplex,
        RST: OutputPin<Error = core::convert::Infallible>,
        DC: OutputPin<Error = core::convert::Infallible>,
    > super::EGDisplay for Ili9341Display<T, M, RST, DC>
{
    /// Writes a string to a specified display segment using the provided font
    /// style (you can use default `DEFAULT_STYLE_SMALL/MID/LARGE`).
    ///
    /// # Arguments
    /// * `segment` - The segment of the display where the text will be written.
    /// * `text` - The string to write to the display.
    /// * `font` - The font style to use for rendering the text.
    fn write_string_to_segment(
        &mut self,
        segment: DisplaySegment,
        text: &str,
        font: MonoTextStyle<Rgb565>,
    ) {
        let size = self.inner.size();
        let segment_size = Size::new(size.width / 2, size.height / 2);

        let (x, y, width, height) = match segment {
            DisplaySegment::TopLeft => (0, 0, segment_size.width, segment_size.height),
            DisplaySegment::TopRight => (
                segment_size.width as i32,
                0,
                segment_size.width,
                segment_size.height,
            ),
            DisplaySegment::BottomLeft => (
                0,
                segment_size.height as i32,
                segment_size.width,
                segment_size.height,
            ),
            DisplaySegment::BottomRight => (
                segment_size.width as i32,
                segment_size.height as i32,
                segment_size.width,
                segment_size.height,
            ),
            DisplaySegment::Center => (
                (size.width / 4) as i32,
                (size.height / 4) as i32,
                segment_size.width,
                segment_size.height,
            ),
        };

        let char_size = font.font.character_size;
        let text_length = text.len() as i32 * char_size.width as i32;

        let section_name_height = char_size.height as i32 + 15;

        // Not forget to clean previous string (just white rect?)
        let clear_rect = Rectangle::new(
            Point::new(x, y + section_name_height),
            Size::new(width, height - section_name_height as u32),
        );
        let clear_style = PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::WHITE)
            .build();

        clear_rect
            .into_styled(clear_style)
            .draw(&mut self.inner)
            .unwrap();

        let text_start = Point::new(
            x + (width as i32 - text_length) / 2,
            y + (height as i32 - char_size.height as i32) / 2,
        );

        Text::with_text_style(
            text,
            text_start,
            font,
            TextStyleBuilder::new().baseline(Baseline::Top).build(),
        )
        .draw(&mut self.inner)
        .unwrap();
    }

    /// Writes a section name to a specified display segment using the provided
    /// font style (you can use default `DEFAULT_STYLE_SMALL/MID/LARGE`). This
    /// is typically used for labeling sections of the display, such as
    /// headers or titles.
    ///
    /// # Arguments
    /// * `segment` - The segment of the display where the section name will be
    ///   written.
    /// * `name` - The name to write to the display.
    /// * `font` - The font style to use for rendering the name.

    fn write_section_name(
        &mut self,
        segment: DisplaySegment,
        name: &str,
        font: MonoTextStyle<Rgb565>,
    ) {
        let size = self.inner.size();
        let segment_size = Size::new(size.width / 2, size.height / 2);

        let (x, y, width, _) = match segment {
            DisplaySegment::TopLeft => (0, 0 + 15, segment_size.width, segment_size.height),
            DisplaySegment::TopRight => (
                segment_size.width as i32,
                0 + 15,
                segment_size.width,
                segment_size.height,
            ),
            DisplaySegment::BottomLeft => (
                0,
                segment_size.height as i32 + 15,
                segment_size.width,
                segment_size.height,
            ),
            DisplaySegment::BottomRight => (
                segment_size.width as i32,
                segment_size.height as i32 + 15,
                segment_size.width,
                segment_size.height,
            ),
            DisplaySegment::Center => (
                (size.width / 4) as i32,
                (size.height / 4) as i32 + 15,
                segment_size.width,
                segment_size.height,
            ),
        };

        let text_size = font.font.character_size;
        let text_length = name.len() as i32 * text_size.width as i32;

        let text_start = Point::new(x + (width as i32 - text_length) / 2, y);

        Text::with_text_style(
            name,
            text_start,
            font,
            TextStyleBuilder::new().baseline(Baseline::Top).build(),
        )
        .draw(&mut self.inner)
        .unwrap();
    }
}

impl<
        T: _esp_hal_spi_master_Instance + 'static,
        M: IsFullDuplex,
        RST: OutputPin<Error = core::convert::Infallible>,
        DC: OutputPin<Error = core::convert::Infallible>,
    > super::Display for Ili9341Display<T, M, RST, DC>
{
    /// Sets a single pixel on the display
    ///
    /// # Arguments
    /// * `x` - The x coordinate of the pixel.
    /// * `y` - The y coordinate of the pixel.
    fn set_pixel(&mut self, x: usize, y: usize) {
        let point = Point::new(x as i32, y as i32);
        let color = Rgb565::BLACK; // The color used for the pixel
        Pixel(point, color).draw(&mut self.inner).unwrap();
    }

    /// Writes a string to the center segment of the display using a mid-sized
    /// default font style.
    //
    /// # Arguments
    /// * `s` - The string to be written on the display.
    fn write_str(&mut self, s: &str) {
        use super::EGDisplay;
        self.write_string_to_segment(DisplaySegment::Center, s, DEFAULT_STYLE_MID);
    }

    /// Resets the display, filling it with a white color.
    ///
    /// This can be used to clear the display before drawing new items.
    fn reset(&mut self) {
        self.inner.clear(Rgb565::WHITE).unwrap();
    }
}
