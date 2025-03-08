# CHIP-8 Emulator

A CHIP-8 emulator written in Rust using SDL2 for graphics and audio.

## Features

- Full CHIP-8 instruction set implementation
- Real-time emulation with configurable speed
- Sound support with square wave audio
- Keyboard input mapping
- Display output with configurable scaling
- Cycle-accurate timing

## Controls

The keyboard layout follows the original CHIP-8 keypad:

```
Original   ->  Keyboard
1 2 3 C    ->  1 2 3 4
4 5 6 D    ->  A Z E R
7 8 9 E    ->  Q S D F
A 0 B F    ->  W X C V
```

## Building

Make sure you have Rust and Cargo installed. Then:

```bash
cargo build --release
```

## Running

To run a ROM:

```bash
cargo run --release -- path/to/rom
```

## Project Structure

- `chip8_impl/`: Core emulator implementation
- `frontDesktop/`: SDL2-based frontend
  - Handles display, input, and audio

## Technical Details

- CPU frequency: 500Hz
- Timer frequency: 60Hz
- Display: 64x32 pixels
- Memory: 4KB
- 16 general purpose registers
- 16-level stack
- 16-key hexadecimal keypad

## Dependencies

- SDL2 for graphics and audio
- Rust 2021 edition

## License

MIT License
