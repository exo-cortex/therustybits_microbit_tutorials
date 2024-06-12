# About

Follow-along repository for embedded Rust tutorial video series [The Rusty Bits](https://www.youtube.com/@therustybits) on Youtube.

This is an embedded Rust project that utilizes the BBC Microbit microcontroller board. The microbit is a microcontroller board with a 5x5 LED-matrix, sensors, a microfone, two buttons and a speaker. It uses an nRF52833 microprocessor. 

## 1 Debug message
It simply runs a loop that outputs a debug message using [rtt-target](https://docs.rs/rtt-target/latest/rtt_target/) (Real-Time Transfer protocol) which should appear in your terminal.

## 2 Blinky-LED
### a manually setup
Configure GPIO pins manually according to nRF52833 product specification document using unsafe rust and pointers. 
### b using `nrf52833-pac` the PAC (peripheral access crate)
Configure GPIOs using the peripherals provided by the PAC.
### c using `nrf52833-hal` the HAL (hardware abstraction layer)
Configure GPIOs using the hardware abstraction layer which in turn accesses the pac.
### d using `microbit-v2` - the BSP (board support package) for the microbit
Using  the board support packaget for the microbit we can use everything even more ergonomically.
### e use the `microbit-v2`'s display struct
Displaying 2 different 5x5 1bit images on the microbit.

## 3 little 2d turing machine
A turing machine with a 2-dimensional (periodic) memory field (a 5x5 array) instead of the usual 1-dimensional ribbon. In it's default version it has 3 states and 2 symbols (On/Off). After 128 steps of iteration the machine resets and its configuration is mutated by randomizing one element in the instructions table (see illustration).
The turing machine has a current state and reads a symbol at the current head position. For every combination of current state and currently read symbol the machine will 1. write a new symbol at the current head's position, move its head and change into a new state. In the 2 dimensional case the head's move directions are UP, LEFT, DOWN, RIGHT. One possible machine configuration is the following:

(current state, read symbol) -> (new state, new symbol, head direction)
(A, 0) -> ( A , 0, U )
(B, 1) -> ( C , 1, D )
(C, 0) -> ( A , 0, R )
(A, 1) -> ( B , 0, L )
(B, 0) -> ( C , 1, L )
(C, 1) -> ( B , 1, D )

For 3 states, 2 symbols and 4 head directions there are 3 * 2 * 3 * 2 * 4 = 36 * 4 = 144 possible turing machines.



# Setup
- install the cargo plugin `cargo-embed` using `$cargo install cargo-embed` (may take a while)

# Running the program:
Connect your microbit and run the command `$ cargo embed`. 

# Links
- (microbit schematic)[https://github.com/microbit-foundation/microbit-v2-hardware/blob/main/V2.21/MicroBit_V2.2.1_nRF52820%20schematic.PDF]
