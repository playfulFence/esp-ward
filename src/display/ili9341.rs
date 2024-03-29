use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::*,
    image::Image,
    mono_font::{MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::*,
    prelude::*,
    primitives::*,
    text::*,
};
use embedded_hal::{blocking::spi::Write, digital::v2::OutputPin};
use esp_hal::{
    delay::Delay,
    spi,
    spi::{master::prelude::_esp_hal_spi_master_Instance, IsFullDuplex},
};
use mipidsi::options::{ColorOrder, Orientation};
use profont::{PROFONT_14_POINT, PROFONT_18_POINT, PROFONT_24_POINT};

use super::DisplaySegment;

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

pub struct Ili9341Display<
    T: _esp_hal_spi_master_Instance + 'static,
    M: IsFullDuplex,
    RST: OutputPin<Error = core::convert::Infallible>,
    DC: OutputPin<Error = core::convert::Infallible>,
> {
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
    fn write_string_to_segment(
        &mut self,
        segment: DisplaySegment,
        text: &str,
        font: MonoTextStyle<Rgb565>,
    ) {
        let size = self.inner.size();
        // Calculate the size for each segment when the display is divided into four
        // equal parts.
        let segment_size = Size::new(size.width / 2, size.height / 2); // for 4 segments

        // Depending on the segment enum, calculate the points (x, y)
        // and the width and height for the segment where the text will be drawn.
        let (x, y, width, height) = match segment {
            // Top-left segment uses the origin point (0,0)
            DisplaySegment::TopLeft => (0, 0, segment_size.width, segment_size.height),
            // Top-right segment starts after the width of the first segment
            DisplaySegment::TopRight => (
                segment_size.width as i32,
                0,
                segment_size.width,
                segment_size.height,
            ),
            // Bottom-left segment starts after the height of the first segment
            DisplaySegment::BottomLeft => (
                0,
                segment_size.height as i32,
                segment_size.width,
                segment_size.height,
            ),
            // Bottom-right segment starts after the width and height of the first segment
            DisplaySegment::BottomRight => (
                segment_size.width as i32,
                segment_size.height as i32,
                segment_size.width,
                segment_size.height,
            ),
            // Center segment is calculated by halving the width and height of the display
            // and then using those as starting coordinates.
            DisplaySegment::Center => (
                (size.width / 4) as i32,
                (size.height / 4) as i32,
                segment_size.width,
                segment_size.height,
            ),
        };

        // Determine the character size of the font used for the text.
        let char_size = font.font.character_size;
        // Calculate the total length of the text based on the number of characters and
        // the width of each character.
        let text_length = text.len() as i32 * char_size.width as i32;

        // Calculate the starting point to draw the text.
        // The x coordinate is the horizontal center of the segment minus half of the
        // text length. The y coordinate is the vertical center of the segment
        // minus half of the text height.
        let text_start = Point::new(
            x + (width as i32 - text_length) / 2,
            y + (height as i32 - char_size.height as i32) / 2,
        );

        // Draw the text onto the display with the calculated starting point,
        // using the specified font and baseline alignment.
        Text::with_text_style(
            text,
            text_start,
            font,
            TextStyleBuilder::new().baseline(Baseline::Top).build(),
        )
        .draw(&mut self.inner)
        .unwrap();
    }

    fn write_section_name(
        &mut self,
        segment: DisplaySegment,
        name: &str,
        font: MonoTextStyle<Rgb565>,
    ) {
        let size = self.inner.size();
        let segment_size = Size::new(size.width / 2, size.height / 2); // for 4 segments plus the center segment

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

        // The height of the text is used to offset the y position so that the section
        // name appears at the top.
        let text_size = font.font.character_size;
        let text_length = name.len() as i32 * text_size.width as i32;

        // Calculate the starting point for the section name.
        // The x coordinate is the horizontal center of the segment minus half of the
        // text length. The y coordinate is very close to the top of the
        // segment.
        let text_start = Point::new(
            x + (width as i32 - text_length) / 2,
            y, // This could be adjusted to add some padding if necessary.
        );

        // Draw the section name at the calculated position.
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
