#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

rustup target add wasm32-unknown-unknown

cargo build \
  --manifest-path "$SCRIPT_DIR/plugin/Cargo.toml" \
  --target wasm32-unknown-unknown \
  --release

cp "$SCRIPT_DIR/plugin/target/wasm32-unknown-unknown/release/typed_scores_plugin.wasm" \
   "$SCRIPT_DIR/src/plugin.wasm"

echo "WASM plugin built and copied to src/plugin.wasm"
