#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::board::Board;
use microbit::display::blocking::Display;
use microbit::hal::timer::Timer;
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

const BLINK_DELAY: u32 = 30;
const SEQUENCE: [(usize, usize); 17] = [
    (0, 0),
    (0, 1),
    (0, 2),
    (0, 3),
    (0, 4),
    (1, 4),
    (2, 4),
    (3, 4),
    (4, 4),
    (4, 3),
    (4, 2),
    (4, 1),
    (4, 0),
    (3, 0),
    (2, 0),
    (1, 0),
    (0, 0),
];

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut light_map = [
        [1, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    loop {
        for elt in SEQUENCE.windows(2) {
            display.show(&mut timer, light_map, BLINK_DELAY);
            let previous = elt[0];
            let current = elt[1];
            set_light(&mut light_map, previous, false);
            set_light(&mut light_map, current, true);
        }
    }
}

fn set_light(light_map: &mut [[u8; 5]; 5], coord: (usize, usize), light: bool) {
    light_map[coord.0][coord.1] = light as u8;
}
