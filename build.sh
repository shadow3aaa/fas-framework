#!/bin/bash
export RUSTFLAGS="-C target-feature=+crt-static"
cargo build --target aarch64-unknown-linux-musl --release
sstrip ./target/aarch64-unknown-linux-musl/release/fas-framework