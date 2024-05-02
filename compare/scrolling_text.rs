// Source code for this showcase example taken from: https://wokwi.com/projects/344869867332043347

#![no_std]
#![no_main]

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
use esp_max7219_nostd::{
    clear_with_state,
    draw_point,
    mappings::SingleDisplayData,
    prepare_display,
    remove_gaps_in_display_text,
    show_moving_text_in_loop,
};
use esp_println::println;
// Display and graphics stuff
use max7219::connectors::PinConnector;
use max7219::{DecodeMode, MAX7219};

#[entry]
fn main() -> ! {
    extern crate alloc;
    #[global_allocator]
    static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

    const HEAP_SIZE: usize = 32 * 1024;
    static mut HEAP: core::mem::MaybeUninit<[u8; HEAP_SIZE]> = core::mem::MaybeUninit::uninit();

    unsafe {
        ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE);
    }

    let peripherals = Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let mut clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let mut delay = Delay::new(&clocks);

    let din = io.pins.gpio23.into_push_pull_output();
    let cs = io.pins.gpio15.into_push_pull_output();
    let clk = io.pins.gpio18.into_push_pull_output();

    let mut display = MAX7219::from_pins(7, din, cs, clk).unwrap();
    // this variable will contain actual configuration of display (which points are
    // lit)
    prepare_display(&mut display, 7, 0x5);
    show_moving_text_in_loop(&mut display, "Hello, Espressif!", 7, 25, 2, &mut delay);

    loop {}
}
