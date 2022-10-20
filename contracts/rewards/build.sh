#!/bin/sh

echo ">> Building rewards contract"

rustup target add wasm32-unknown-unknown
cargo build --all --target wasm32-unknown-unknown --release
