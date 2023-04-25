#!/bin/bash
DIR="${0%/*}"
ELF="$DIR/target/aarch64-unknown-linux-musl/release/fas-framework"
export RUSTFLAGS="-C target-feature=+crt-static -C target-cpu=cortex-a78 -C target-cpu=cortex-a55"
cargo build --target aarch64-unknown-linux-musl --release
sstrip "$ELF"