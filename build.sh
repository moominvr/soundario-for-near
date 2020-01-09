#!/bin/bash
set -e

cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/soundario.wasm ./res/
#wasm-opt -Oz --output ./res/soundario.wasm ./res/soundario.wasm
rm -rf target
