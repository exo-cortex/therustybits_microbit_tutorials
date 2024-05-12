# About

Follow-along repository for embedded Rust tutorial video series (The Rusty Bits)[https://www.youtube.com/@therustybits] on Youtube.

This is an embedded Rust project that utilizes the BBC Microbit microcontroller board. The microbit is a microcontroller board with a 5x5 LED-matrix, sensors, a microfone, two buttons and a speaker. It uses an nRF52833 microprocessor. 

## 1 Debug message
It simply runs a loop that outputs a debug message using (rtt-target)[https://docs.rs/rtt-target/latest/rtt_target/] (Real-Time Transfer protocol) which should appear in your terminal.

## 2 Blinky-LED
### a manually setup
Configure GPIO pins manually according to nRF52833 product specification document using unsafe rust and pointers. 
### b using `nrf52833-pac` the PAC (peripheral access crate)
Configure GPIOs using the peripherals provided by the PAC.
### c using `nrf52833-hal` the HAL (hardware abstraction layer)
Configure GPIOs using the hardware abstraction layer which in turn accesses the pac.
### d using `microbit-v2` - the BSP (board support package) for the microbit
Using  the board support packaget for the microbit we can use everything even more ergonomically.

# Setup
- install the cargo plugin `cargo-embed` using `$cargo install cargo-embed` (may take a while)

# Running the program:
Connect your microbit and run the command `$ cargo embed`. 

# Links
- (microbit schematic)[https://github.com/microbit-foundation/microbit-v2-hardware/blob/main/V2.21/MicroBit_V2.2.1_nRF52820%20schematic.PDF]
