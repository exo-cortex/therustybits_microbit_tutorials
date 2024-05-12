#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello, world!");
    let mut x: u32 = 0;
    loop {
        rprintln!("Echo...{}", x);
        for _ in 0..100_000 {
            nop();
        }
        x += 1;
        x %= 10;
    }
}
