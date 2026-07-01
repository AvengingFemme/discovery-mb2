#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{InputPin, OutputPin};
use microbit::display::blocking::Display;
use microbit::{hal::gpio, hal::timer::Timer, Board};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);

    let mut button_a = board.buttons.button_a;
    let mut button_b = board.buttons.button_b;

    let mut display = Display::new(board.display_pins);

    let neutral_display_bitmap: [[u8; 5]; 5] = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    let left_display_bitmap: [[u8; 5]; 5] = [
        [0, 0, 1, 0, 0],
        [0, 1, 0, 0, 0],
        [1, 1, 1, 1, 1],
        [0, 1, 0, 0, 0],
        [0, 0, 1, 0, 0],
    ];

    let right_display_bitmap: [[u8; 5]; 5] = [
        [0, 0, 1, 0, 0],
        [0, 0, 0, 1, 0],
        [1, 1, 1, 1, 1],
        [0, 0, 0, 1, 0],
        [0, 0, 1, 0, 0],
    ];

    let blank_display_bitmap: [[u8; 5]; 5] = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    let mut seq_count = 0;
    let seq_count_max = 100;

    loop {
        if seq_count >= seq_count_max {
            seq_count = 0;
        } else {
            seq_count += 1;
        }

        let left_pressed = button_a.is_low().unwrap();
        let right_pressed = button_b.is_low().unwrap();
        match (left_pressed, right_pressed) {
            (false, false) => {
                display.show(&mut timer, neutral_display_bitmap, 10);
                seq_count = 0
            }
            (true, false) => {
                if seq_count < 60 {
                    display.show(&mut timer, left_display_bitmap, 10);
                } else {
                    display.show(&mut timer, blank_display_bitmap, 10);
                }
            }
            (false, true) => {
                if seq_count < 60 {
                    display.show(&mut timer, right_display_bitmap, 10);
                } else {
                    display.show(&mut timer, blank_display_bitmap, 10);
                }
            }
            (true, true) => {
                display.show(&mut timer, neutral_display_bitmap, 10);
                seq_count = 0
            }
        }
    }
}
