#! /bin/bash

cargo fmt &
cargo clippy &
cargo run -- daily --day "$1"
