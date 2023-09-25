#!/bin/sh

rustc ack.rs --crate-type=cdylib --target=wasm32-unknown-unknown -o ack.wasm
