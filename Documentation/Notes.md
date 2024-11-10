# Rust Notizen

## Used Tools

cargo-generate: use preexisting repository as template
cargo install cargo-generate

cargo-binutils: inspect binaries
cargo install cargo-binutils
rustup component add llvm-tools

qemu-system-arm: sudo apt install qemu-system-arm

probe-rs: debugging

## Embedded Rust Crate Architecture

* **Microcontroller** consists of
  * **Microprocessor** e.g. ARM Cortex M3 is accessed by
    * **Micro-architecture crate** e.g. cortex-m
  * **Peripheral registers** these are accessed by
    * **Peripheral Access Crate (PAC)** e.g. stm32f10x create from SVD file
  * **HAL Crate**  encapsulates the previous two crates and is used by
  * **Board Crates** specific for e.g. the Nucleo64 board

## Toolchain

### Cortex-M3 (ARMv7-M architecture):

rustup target add thumbv7m-none-eabi

## Setting up a new project

```
cargo generate --git https://github.com/rust-embedded/cortex-m-quickstart
```

## USB-zu-Serial-Converter

cp210x von paradisetronic

| Kabel  | Funktion |
| ------ | -------- |
| Rot    | 5 V      |
| Scharz | GND      |
| Grün   | TX       |
| Weiß   | RX       |

## Nucleo64F103RB

### USART

* USART2
* Available via virtual com port ttyACM0
* TX PA2
* RX PA3

* USART3
* TX PC10
* RX PC11