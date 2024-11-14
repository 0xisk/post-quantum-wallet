# Steps:


1. Install `riscv32` arch target for Rust:
```sh
# Add the target
rustup target add riscv32-unknown-unknown

# Verify Installation:
rustup target list --installed

# Build with the Target
cargo build --target riscv32-unknown-unknown
```