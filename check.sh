#!/bin/bash

cargo build &&
    cargo build --no-default-features &&
    cargo test &&
    cargo test --no-default-features &&
    cargo fmt --check &&
    cargo clippy -- --deny warnings &&
    cargo clippy --no-default-features -- --deny warnings