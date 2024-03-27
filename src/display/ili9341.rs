use esp_hal::{spi};

use mipidsi::options::{ Orientation, ColorOrder };
use mipidsi::model::ILI9341Rgb565;

use display_interface_spi::SPIInterfaceNoCS;

use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::*;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::*;
use embedded_graphics::text::*;
use embedded_graphics::image::Image;
use embedded_graphics::geometry::*;
use embedded_graphics::draw_target::DrawTarget;
use embedded_hal::digital::v2::OutputPin;

pub struct Ili9341Display {
    inner: mipidsi::Builder<SPIInterfaceNoCS< , OutputPin>>
}