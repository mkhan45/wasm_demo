#!/bin/sh

rustc ack.rs -O --crate-type=cdylib --target=wasm32-unknown-unknown -o ack.wasm
