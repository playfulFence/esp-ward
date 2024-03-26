use pcd8544::PCD8544;
use core::fmt::Write;

use embedded_hal::digital::v2::OutputPin;
pub struct Pcd8544Display<
    CLK: OutputPin,
    DIN: OutputPin, 
    DC: OutputPin, 
    CE: OutputPin, 
    RST: OutputPin,
    BL: OutputPin>
{
    inner: PCD8544<CLK, DIN, DC, CE, RST, BL>
}

impl<
    CLK: OutputPin<Error = core::convert::Infallible>,
    DIN: OutputPin<Error = core::convert::Infallible>, 
    DC: OutputPin<Error = core::convert::Infallible>, 
    CE: OutputPin<Error = core::convert::Infallible>, 
    RST: OutputPin<Error = core::convert::Infallible>,
    BL: OutputPin<Error = core::convert::Infallible>> Pcd8544Display<CLK, DIN, DC, CE, RST, BL>
{
    pub fn create_display(pin_clk: CLK, pin_din: DIN, pin_dc: DC, pin_ce: CE, pin_rst: RST, mut pin_backlight: BL) 
        -> Pcd8544Display<CLK, DIN, DC, CE, RST, BL>
    {
        let mut display = PCD8544::new(pin_clk, pin_din, pin_dc, pin_ce, pin_rst, pin_backlight).unwrap();
        display.reset().unwrap();
        display.set_light(true);
        Pcd8544Display{inner: display}
    }
}


impl<
    CLK: OutputPin<Error = core::convert::Infallible>,
    DIN: OutputPin<Error = core::convert::Infallible>, 
    DC: OutputPin<Error = core::convert::Infallible>, 
    CE: OutputPin<Error = core::convert::Infallible>, 
    RST: OutputPin<Error = core::convert::Infallible>,
    BL: OutputPin<Error = core::convert::Infallible>> super::Display for Pcd8544Display<CLK, DIN, DC, CE, RST, BL>
{
    fn set_pixel(&mut self, x: usize, y: usize) { 
        self.inner.set_pixel(x.try_into().unwrap(), y.try_into().unwrap()).unwrap();
    }

    fn reset(&mut self) {
        self.inner.reset();
    }

    fn write_str(&mut self, str: &str) {
        self.inner.write_str(str);
    }
}

