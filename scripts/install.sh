#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cargo build --release --manifest-path "$SCRIPT_DIR/../Cargo.toml"

mv "$SCRIPT_DIR/../target/release/tmgitty" "$SCRIPT_DIR/../bin"

