# Rust Notizen

## Used Tools

cargo-generate: use preexisting repository as template
cargo install cargo-generate

cargo-binutils: inspect binaries
cargo install cargo-binutils
rustup component add llvm-tools


qemu-system-arm: sudo apt install qemu-system-arm

probe-rs: debugging

## Toolchain
# Cortex-M3 (ARMv7-M architecture):

rustup target add thumbv7m-none-eabi

## Setting up a new project

```
cargo generate --git https://github.com/rust-embedded/cortex-m-quickstart
```