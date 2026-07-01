#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{board::Board, display::blocking::Display, hal::Timer};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut seq_position: u8 = 0;
    loop {
        // Show light_it_all for 1000ms
        display.show(&mut timer, matrix_for_seq_position(seq_position), 50);
        // clear the display again
        if seq_position >= 15 {
            seq_position = 0;
        } else {
            seq_position += 1;
        }
    }
}

fn matrix_for_seq_position(seq_position: u8) -> [[u8; 5]; 5] {
    assert!(seq_position <= 15);
    let mut matrix = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    match seq_position {
        0 => matrix[0][0] = 1,
        1 => matrix[0][1] = 1,
        2 => matrix[0][2] = 1,
        3 => matrix[0][3] = 1,
        4 => matrix[0][4] = 1,
        5 => matrix[1][4] = 1,
        6 => matrix[2][4] = 1,
        7 => matrix[3][4] = 1,
        8 => matrix[4][4] = 1,
        9 => matrix[4][3] = 1,
        10 => matrix[4][2] = 1,
        11 => matrix[4][1] = 1,
        12 => matrix[4][0] = 1,
        13 => matrix[3][0] = 1,
        14 => matrix[2][0] = 1,
        15 => matrix[1][0] = 1,
        16..=u8::MAX => panic!("invalid seq_position encountered"),
    }

    matrix
}
