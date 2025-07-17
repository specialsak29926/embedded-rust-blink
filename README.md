# hello-rust: Raspberry Pi Pico (RP2040) LED Blink Example

This project demonstrates how to blink the onboard LED on a Raspberry Pi Pico (RP2040) using Rust.

## Features
- Blinks the onboard LED (GPIO 25) on the Raspberry Pi Pico
- Uses `rp-pico` and `rp2040-hal` crates
- Suitable as a template for embedded Rust development on the RP2040

## Prerequisites
- Rust toolchain (latest stable)
- Rust target for ARM Cortex-M0+:
  ```
  rustup target add thumbv6m-none-eabi
  ```
- [elf2uf2-rs](https://crates.io/crates/elf2uf2-rs) for converting ELF to UF2:
  ```
  cargo install elf2uf2-rs
  ```
- Raspberry Pi Pico board

## Project Structure
- `src/main.rs` — Main application (LED blink logic)
- `memory.x` — Linker script for RP2040 memory layout
- `.cargo/config.toml` — Cargo configuration for correct linking
- `build.rs` — Ensures `memory.x` is included in the build

## Importance of `memory.x` and `build.rs`

- **`memory.x`**: This linker script defines the memory layout for the RP2040 microcontroller. It tells the Rust compiler and linker where the flash and RAM are located and how large they are. Without this file, the program cannot be correctly placed in memory, and the resulting binary will not run on the Pico.

- **`build.rs`**: This build script ensures that `memory.x` is always included in the build process. It notifies Cargo to rerun the build if `memory.x` changes and helps the linker find the script. Without `build.rs`, the linker might not use the correct memory layout, leading to errors like "entry point is not in mapped part of file" or a non-functional binary.

## Building the Project
1. **Clean and build for the Pico target:**
   ```
   cargo clean
   cargo build --release --target=thumbv6m-none-eabi
   ```
2. **Convert the ELF to UF2:**
   ```
   elf2uf2-rs target/thumbv6m-none-eabi/release/hello-rust target/thumbv6m-none-eabi/release/hello-rust.uf2
   ```

## Flashing to the Pico
1. Hold down the **BOOTSEL** button on your Pico and plug it into your computer via USB.
2. The Pico will appear as a USB drive.
3. Copy `hello-rust.uf2` to the Pico drive.
4. The Pico will reboot and start blinking the onboard LED!

## Troubleshooting
- If you see `Error: "entry point is not in mapped part of file"`, ensure `memory.x` is present in the project root and that you have a `build.rs` file.
- Make sure you are using the correct Rust target and have installed all prerequisites.

## References
- [rp-hal Project Template](https://github.com/rp-rs/rp2040-project-template)
- [Rust Embedded Book](https://docs.rust-embedded.org/book/)

