#! /bin/bash

cargo fmt &
cargo clippy &
cargo run src/main.rs
