# About

Follow-along repository for embedded Rust tutorial video series (The Rusty Bits)[https://www.youtube.com/@therustybits] on Youtube.

This is an embedded Rust project that utilizes the BBC Microbit microcontroller board. The microbit is a microcontroller board with a 5x5 LED-matrix, sensors, a microfone, two buttons and a speaker. It uses an nRF52833 microprocessor. 

## 1 Debug message
It simply runs a loop that outputs a debug message using (rtt-target)[https://docs.rs/rtt-target/latest/rtt_target/] (Real-Time Transfer protocol) which should appear in your terminal.

# Setup
- install the cargo plugin `cargo-embed` using `$cargo install cargo-embed` (may take a while)

# Running the program:
Connect your microbit and run the command `$ cargo embed`. 

# Links
- (microbit schematic)[https://github.com/microbit-foundation/microbit-v2-hardware/blob/main/V2.21/MicroBit_V2.2.1_nRF52820%20schematic.PDF]
