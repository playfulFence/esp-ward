// Source code for this showcase example taken from: https://wokwi.com/projects/342697409287029332

#![no_std]
#![no_main]

use esp_hal::{
    adc::{AdcConfig, Attenuation, ADC},
    clock::ClockControl,
    peripherals::{ Peripherals, ADC2 },
    gpio::*,
    prelude::*,
    spi,
    timer::TimerGroup,
    Rtc,
    IO,
    Delay,
};

/* Set this number according to count of displays 
in chain (check "chain" parameter of max7219-matrix in diagram.json) */
const DISPLAY_COUNT : usize = 7;

use embedded_hal;

use max7219::connectors::PinConnector;
use max7219::MAX7219;
use max7219::DecodeMode;

use esp_max7219_nostd::{prepare_display, show_moving_text_in_loop, draw_point, clear_with_state};
use esp_max7219_nostd::mappings::SingleDisplayData;

use esp_backtrace as _;
use esp_println::println;

#[derive(Copy, Clone, PartialEq)]
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
    pub fn check(&mut self){
        self.pressed = !self.button.is_low().unwrap();
    }

    pub fn poll(&mut self, delay :&mut Delay) -> Event {
        let pressed_now = !self.button.is_low().unwrap();
        if !self.pressed  &&  pressed_now
        {
            delay.delay_ms(30 as u32);
            self.check();
            if !self.button.is_low().unwrap() {
                Event::Pressed
            }
            else {
                Event::Nothing
            }
        }
        else if self.pressed && !pressed_now{
            delay.delay_ms(30 as u32);
            self.check();
            if self.button.is_low().unwrap()
            {
                Event::Released
            }
            else {
                Event::Nothing
            }
        }
        else{
            Event::Nothing
        }
        
    }
}

extern crate alloc;
use alloc::vec::Vec;
use core::mem::MaybeUninit;
#[global_allocator]
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

fn init_heap() {
    const HEAP_SIZE: usize = 32 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE);
    }
}

#[entry]
fn main() -> ! {
    init_heap();
    let peripherals = Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let mut clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.LPWR);
    let timer_group0 = TimerGroup::new(
        peripherals.TIMG0,
        &clocks
    );
    let mut wdt0 = timer_group0.wdt;
    let timer_group1= TimerGroup::new(
        peripherals.TIMG1,
        &clocks
    );
    let mut wdt1 = timer_group1.wdt;
    
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let mut delay = Delay::new(&clocks);

    /* Setting up environment for joystick */
    let mut adc2_config = AdcConfig::new();
    
    /* Setting up joystick's axles and select button */
    let mut joystick_select = Button::new(io.pins.gpio21.into_pull_up_input()); // MENU button on esp32-s3-usb-otg (low level when pressed)
    let mut y_axis = adc2_config.enable_pin(io.pins.gpio25.into_analog(), Attenuation::Attenuation11dB);
    let mut x_axis = adc2_config.enable_pin(io.pins.gpio2.into_analog(), Attenuation::Attenuation11dB);

    let mut adc2 = ADC::<ADC2>::new(peripherals.ADC2, adc2_config);

    let mut y_axis_actual :u16 = 0;
    let mut x_axis_actual :u16 = 0;

    /* Set pins for display initialisation */
    let din = io.pins.gpio23.into_push_pull_output();
    let cs = io.pins.gpio15.into_push_pull_output();
    let clk = io.pins.gpio18.into_push_pull_output();

    let mut display = MAX7219::from_pins(DISPLAY_COUNT, din, cs, clk).unwrap();


    /* This vector will contain actual configurations of each display (which points are lit) */
    let mut display_actual_state : Vec<[u8;8]> = Vec::new();

    let mut tmp = [0b00000000 as u8;8];

    for i in 0..DISPLAY_COUNT
    {
        display_actual_state.push(tmp);
        tmp = [0b00000000 as u8;8];
    }

    prepare_display(&mut display, DISPLAY_COUNT, 0x5);
    draw_point(&mut display, 0, &mut display_actual_state[0], 1, 1); // draw starting point

    let mut x : usize = 1;
    let mut y : usize = 1;
    let mut dp_index : usize = 0;  // index of actual display

    loop {
        /* Little delay to make movement smoother */
        delay.delay_ms(40 as u32); 

        /* Read analog data from x and y joystick pins */
        y_axis_actual = nb::block!(adc2.read(&mut y_axis)).unwrap();
        x_axis_actual = nb::block!(adc2.read(&mut x_axis)).unwrap();

        /* SELECT button will clear whole display and set you to a start point */
        if let Event::Pressed = joystick_select.poll(&mut delay) 
        {   
            println!("Reset! Now pointer is at ({}({});{})(display No.{})", x, x + 8*dp_index, y, dp_index);
            for i in 0..DISPLAY_COUNT {clear_with_state(&mut display, i, &mut display_actual_state[i]);}
            x = 1;
            y = 1; 
            dp_index = 0; 
            draw_point(&mut display, dp_index, &mut display_actual_state[dp_index], x, y);
        }
        if x_axis_actual < 2048 // right
        {
            if x < 8 && (dp_index < (DISPLAY_COUNT)) {x += 1;}
            else if x == 8 && dp_index != (DISPLAY_COUNT - 1)
            {
                x = 1;
                dp_index += 1;
            }
            draw_point(&mut display, dp_index, &mut display_actual_state[dp_index], x, y);
            println!("Moved right!\nNow pointer is at ({}({});{})(display No.{})\n\n", x, x + 8*dp_index, y, dp_index);
        }
        if x_axis_actual > 2048 // left
        {
            if (x == 1 && dp_index == 0) {continue;}
            else if (x == 0 && dp_index != 0)
            {
                x = 8;
                dp_index -= 1;
            }
            else {x -= 1;}
            
            draw_point(&mut display, dp_index, &mut display_actual_state[dp_index], x, y);
            println!("Moved left!\nNow pointer is at ({}({});{})(display No.{})\n\n", x, x + 8*dp_index, y, dp_index);
        }
        if y_axis_actual < 2048 // down
        {
            if y < 8 
            {
                y += 1;
                draw_point(&mut display, dp_index, &mut display_actual_state[dp_index], x, y);
            }
            println!("Moved down!\nNow pointer is at ({}({});{})(display No.{})\n\n", x, x + 8*dp_index, y, dp_index);
        }
         if y_axis_actual > 2048 // up
        {
            if y > 1 
            {
                y -= 1;
                draw_point(&mut display, dp_index, &mut display_actual_state[dp_index], x, y);
            }
            println!("Moved up!\nNow pointer is at ({}({});{})(display No.{})\n\n", x, x + 8*dp_index, y, dp_index);
        }
    }
}