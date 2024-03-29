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
use profont::{PROFONT_18_POINT, PROFONT_24_POINT};

pub const DEFAULT_STYLE_MID: MonoTextStyle<Rgb565> = MonoTextStyleBuilder::new()
    .font(&PROFONT_18_POINT)
    .text_color(RgbColor::BLACK)
    .build();

pub const DEFAULT_STYLE_BIG: MonoTextStyle<Rgb565> = MonoTextStyleBuilder::new()
    .font(&PROFONT_24_POINT)
    .text_color(RgbColor::BLACK)
    .build();

pub enum DisplaySegment {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

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

    pub fn write_string_to_segment(
        &mut self,
        segment: DisplaySegment,
        text: &str,
        font: MonoTextStyle<Rgb565>,
    ) {
        let (x, y, width, height) = match segment {
            DisplaySegment::TopLeft => (0, 0, 120, 160),
            DisplaySegment::TopRight => (120, 0, 120, 160),
            DisplaySegment::BottomLeft => (0, 160, 120, 160),
            DisplaySegment::BottomRight => (120, 160, 120, 160),
            DisplaySegment::Center => (60, 80, 120, 160), // Center segment
        };

        // Calculate text position; this example centers the text in the segment
        let text_size = font.font.character_size;
        let text_start = Point::new(
            x + (width as i32 - text.len() as i32 * text_size.width as i32) / 2,
            y + (height as i32 - text_size.height as i32) / 2,
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
}
