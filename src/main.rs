#![no_std]
#![no_main]
// #![feature(generic_const_exprs)]

use rand::{rngs::SmallRng, Rng, SeedableRng};

use panic_halt as _;

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use microbit::{
    board::Board,
    display::blocking::Display,
    gpio::MicrophonePins,
    hal::{comp::Comp, timer::Timer},
};

// directions: 4 = {U, L, D, R}
// symbols: 2 = {0, 1}
// states: 3, {1, 2, 3}
/// ( , ) -> (D, new_state, new_sym)
/// ( , ) -> (D, new_state, new_sym)
/// ( , ) -> (D, new_state, new_sym)
/// ( , ) -> (D, new_state, new_sym)
/// ( , ) -> (D, new_state, new_sym)
/// ( , ) -> (D, new_state, new_sym)
#[derive(Copy, Clone, Default)]
struct Rule {
    pub current_state: u8,
    pub current_symbol: u8,
    pub direction: u8,
    pub next_state: u8,
    pub next_symbol: u8,
}

impl Rule {
    pub fn new(
        current_state: u8,
        current_symbol: u8,
        direction: u8,
        next_state: u8,
        next_symbol: u8,
    ) -> Self {
        Rule {
            current_state,
            current_symbol,
            direction,
            next_state,
            next_symbol,
        }
    }
}

const NUM_STATES: usize = 3;
const NUM_SYMBOLS: usize = 2;
const NUM_DIRECTIONS: usize = 4;

struct Ruleset {
    rules: [Rule; 6],
}

impl Ruleset {
    fn new(rng: &mut SmallRng) -> Self {
        let mut temp_ruleset = [Rule::default(); 6];
        for current_state in 0..NUM_STATES {
            for current_symbol in 0..NUM_SYMBOLS {
                temp_ruleset[current_state * NUM_SYMBOLS + current_symbol] = Rule::new(
                    current_state as u8,
                    current_symbol as u8,
                    rng.gen_range(0..NUM_DIRECTIONS) as u8,
                    rng.gen_range(0..NUM_STATES) as u8,
                    rng.gen_range(0..NUM_SYMBOLS) as u8,
                )
            }
        }
        Ruleset {
            rules: temp_ruleset,
        }
    }

    fn get_instructions(&self, current_state: u8, current_symbol: u8) -> (u8, u8, u8) {
        let rule = &self.rules[(current_state as usize) * NUM_SYMBOLS + current_symbol as usize];
        (rule.direction, rule.next_state, rule.next_symbol)
    }
}

struct TuringMachine {
    head: (u8, u8),
    memory: [[u8; 5]; 5],
    state: u8,
    ruleset: Ruleset,
}

impl TuringMachine {
    fn new(rng: &mut SmallRng) -> Self {
        TuringMachine {
            head: (2, 2),
            memory: [[0; 5]; 5],
            state: 0,
            ruleset: Ruleset::new(rng),
        }
    }

    fn read_symbol(&self) -> u8 {
        self.memory[self.head.0 as usize][self.head.1 as usize]
    }
    fn write_symbol(&mut self, symbol: u8) {
        self.memory[self.head.0 as usize][self.head.1 as usize] = symbol;
    }

    fn update(&mut self) {
        let instructions = self
            .ruleset
            .get_instructions(self.state, self.read_symbol());
        self.state = instructions.1;
        self.write_symbol(instructions.2);
        match instructions.0 {
            0 => {
                self.head.0 += 1;
                self.head.0 %= 5;
            }
            1 => {
                self.head.1 += 1;
                self.head.1 %= 5;
            }
            2 => {
                self.head.0 += 4;
                self.head.0 %= 5;
            }
            3 => {
                self.head.1 += 4;
                self.head.1 %= 5;
            }
            _ => panic!(),
        }
    }
}

fn fill_rand(array: &mut [[u8; 5]; 5], rng: &mut SmallRng) {
    for row in array {
        for col in row {
            *col = rng.gen_range(0..=1);
        }
    }
}

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut rng = SmallRng::seed_from_u64(0);

    #[allow(non_snake_case)]
    let letter_X = [
        [1, 0, 0, 0, 1],
        [0, 1, 0, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 1, 0, 1, 0],
        [1, 0, 0, 0, 1],
    ];
    let letter_O = [
        [0, 1, 1, 1, 0],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [0, 1, 1, 1, 0],
    ];

    let mut array_rng = [[0; 5]; 5];
    fill_rand(&mut array_rng, &mut rng);

    loop {
        // let _ = display.show(&mut timer, letter_O, 100);
        // let _ = display.show(&mut timer, letter_X, 100);
        let _ = display.show(&mut timer, array_rng, 100);
        fill_rand(&mut array_rng, &mut rng);
    }
}
