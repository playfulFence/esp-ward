#!/usr/bin/env bash

# Build the documentation with the specified features and target
cargo +nightly doc --no-deps --features=esp32c6-mqtt,docs --target=riscv32imac-unknown-none-elf

# Create a new directory for the documentation and copy the generated docs into it
mkdir -p ./docs
cp -R ../target/riscv32imac-unknown-none-elf/doc/* ./docs/
