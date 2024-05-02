// Source code for this showcase example taken from: https://wokwi.com/projects/341706650098336338

#![no_std]
#![no_main]

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
use embedded_hal;
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    gpio::*,
    peripherals::Peripherals,
    prelude::*,
    spi,
    timer::TimerGroup,
    Delay,
    Rtc,
    IO,
};
use esp_println::println;
use mipidsi::options::{ColorOrder, Orientation};
use profont::{PROFONT_18_POINT, PROFONT_24_POINT};

const textStyle: TextStyle = TextStyleBuilder::new()
    .alignment(embedded_graphics::text::Alignment::Center)
    .baseline(embedded_graphics::text::Baseline::Middle)
    .build();

// Debouncing algorythm
pub enum Event {
    Pressed,
    Released,
    Nothing,
}
pub struct Button<T> {
    button: T,
    pressed: bool,
}
impl<T: ::embedded_hal::digital::v2::InputPin<Error = core::convert::Infallible>> Button<T> {
    pub fn new(button: T) -> Self {
        Button {
            button,
            pressed: true,
        }
    }
    pub fn check(&mut self) {
        self.pressed = !self.button.is_low().unwrap();
    }

    pub fn poll(&mut self, delay: &mut Delay) -> Event {
        let pressed_now = !self.button.is_low().unwrap();
        if !self.pressed && pressed_now {
            delay.delay_ms(30 as u32);
            self.check();
            if !self.button.is_low().unwrap() {
                Event::Pressed
            } else {
                Event::Nothing
            }
        } else if self.pressed && !pressed_now {
            delay.delay_ms(30 as u32);
            self.check();
            if self.button.is_low().unwrap() {
                Event::Released
            } else {
                Event::Nothing
            }
        } else {
            Event::Nothing
        }
    }
}

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();

    let mut system = peripherals.SYSTEM.split();

    let mut clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    println!("About to initialize the SPI LED driver ILI9341");
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // Set corresponding pins
    let mosi = io.pins.gpio7;
    let cs = io.pins.gpio2;
    let rst = io.pins.gpio10;
    let dc = io.pins.gpio3;
    let sck = io.pins.gpio6;
    let miso = io.pins.gpio9;
    let backlight = io.pins.gpio4;

    // Then set backlight (set_low() - display lights up when signal is in 0,
    // set_high() - opposite case(for example.))
    let mut backlight = backlight.into_push_pull_output();
    // backlight.set_low().unwrap();

    // Configure SPI
    let spi = spi::master::Spi::new(
        peripherals.SPI2,
        100u32.MHz(),
        spi::SpiMode::Mode0,
        &mut clocks,
    )
    .with_pins(Some(sck), Some(mosi), Some(miso), Some(cs));

    let di = SPIInterfaceNoCS::new(spi, dc.into_push_pull_output());
    let reset = rst.into_push_pull_output();

    let mut delay = Delay::new(&clocks);

    let mut display = mipidsi::Builder::ili9341_rgb565(di)
        .with_display_size(240 as u16, 320 as u16)
        .with_framebuffer_size(240 as u16, 320 as u16)
        .with_orientation(Orientation::LandscapeInverted(true))
        .with_color_order(ColorOrder::Bgr)
        .init(&mut delay, Some(reset))
        .unwrap();

    println!("Initialized");

    display.clear(Rgb565::WHITE).unwrap();

    let mut button_green = Button::new(io.pins.gpio0.into_pull_up_input());
    let mut button_blue = Button::new(io.pins.gpio1.into_pull_up_input());

    Text::with_text_style(
        "Press GREEN button",
        display.bounding_box().center() - Size::new(0, 35),
        MonoTextStyle::new(&PROFONT_18_POINT, Rgb565::BLACK),
        textStyle,
    )
    .draw(&mut display)
    .unwrap();

    Text::with_text_style(
        "Press BLUE button",
        display.bounding_box().center() + Size::new(0, 25),
        MonoTextStyle::new(&PROFONT_18_POINT, Rgb565::BLACK),
        textStyle,
    )
    .draw(&mut display)
    .unwrap();

    let mut green_cnt = 0;
    let mut blue_cnt = 0;

    let mut last_pressed_blue: bool = false;

    loop {
        if let Event::Pressed = button_green.poll(&mut delay) {
            green_cnt += 1;

            Rectangle::with_center(
                display.bounding_box().center() - Size::new(0, 30),
                Size::new(270, 30),
            )
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .fill_color(Rgb565::WHITE)
                    .stroke_color(Rgb565::WHITE)
                    .stroke_width(1)
                    .build(),
            )
            .draw(&mut display);

            Text::with_text_style(
                "Green button pressed!",
                display.bounding_box().center() - Size::new(0, 35),
                MonoTextStyle::new(&PROFONT_18_POINT, Rgb565::CSS_GREEN),
                textStyle,
            )
            .draw(&mut display)
            .unwrap();

            println!("Green! (x{})", green_cnt);

            if last_pressed_blue {
                Rectangle::with_center(
                    display.bounding_box().center() + Size::new(0, 30),
                    Size::new(270, 30),
                )
                .into_styled(
                    PrimitiveStyleBuilder::new()
                        .fill_color(Rgb565::WHITE)
                        .stroke_color(Rgb565::WHITE)
                        .stroke_width(1)
                        .build(),
                )
                .draw(&mut display);

                Text::with_text_style(
                    "Press BLUE button",
                    display.bounding_box().center() + Size::new(0, 25),
                    MonoTextStyle::new(&PROFONT_18_POINT, Rgb565::BLACK),
                    textStyle,
                )
                .draw(&mut display)
                .unwrap();
            }
            last_pressed_blue = false;
        }
        if let Event::Pressed = button_blue.poll(&mut delay) {
            blue_cnt += 1;

            Rectangle::with_center(
                display.bounding_box().center() + Size::new(0, 30),
                Size::new(270, 30),
            )
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .fill_color(Rgb565::WHITE)
                    .stroke_color(Rgb565::WHITE)
                    .stroke_width(1)
                    .build(),
            )
            .draw(&mut display);

            Text::with_text_style(
                "Blue button pressed!",
                display.bounding_box().center() + Size::new(0, 25),
                MonoTextStyle::new(&PROFONT_18_POINT, Rgb565::BLUE),
                textStyle,
            )
            .draw(&mut display)
            .unwrap();

            println!("Blue! (x{})", blue_cnt);

            if !last_pressed_blue {
                Rectangle::with_center(
                    display.bounding_box().center() - Size::new(0, 30),
                    Size::new(270, 30),
                )
                .into_styled(
                    PrimitiveStyleBuilder::new()
                        .fill_color(Rgb565::WHITE)
                        .stroke_color(Rgb565::WHITE)
                        .stroke_width(1)
                        .build(),
                )
                .draw(&mut display);

                Text::with_text_style(
                    "Press GREEN button",
                    display.bounding_box().center() - Size::new(0, 35),
                    MonoTextStyle::new(&PROFONT_18_POINT, Rgb565::BLACK),
                    textStyle,
                )
                .draw(&mut display)
                .unwrap();
            }
            last_pressed_blue = true;
        }
    }
}
