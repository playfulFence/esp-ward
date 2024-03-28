use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::*,
    image::Image,
    mono_font::MonoTextStyle,
    pixelcolor::*,
    prelude::*,
    primitives::*,
    text::*,
};
use embedded_hal::{blocking::spi::Write, digital::v2::OutputPin};
use esp_hal::{
    spi,
    spi::{master::prelude::_esp_hal_spi_master_Instance, IsFullDuplex},
};
use mipidsi::options::{ColorOrder, Orientation};

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
    pub fn create_display(spi: spi::master::Spi<'static, T, M>) {}
}
