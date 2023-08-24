#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer},
};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut matrix = [
        [1, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    let mut state = [0, 0];

    loop {
        let mut state = iterate(&mut state);
        iterate_matrix_from_state(state, &mut matrix);
        display.show(&mut timer, matrix, 1000);

        display.clear();
        timer.delay_ms(1000_u32);
    }
}

fn iterate_matrix_from_state<'a>(
    state: &[u8; 2],
    matrix: &'a mut [[u8; 5]; 5],
) -> &'a [[u8; 5]; 5] {
    let [x, y] = state;

    // TODO: iter_mut
    matrix
}

fn iterate(state: &mut [u8; 2]) -> &[u8; 2] {
    match state {
        [x, 0] => {
            state[0] = *x + 1;
            state
        }
        [4, y] => {
            state[0] = *y + 1;
            state
        }

        [x, 4] => {
            state[0] = *x - 1;
            state
        }

        [0, y] => {
            state[0] = *y - 1;
            state
        }
        _ => unreachable!(),
    }
}
