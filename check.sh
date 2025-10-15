#!/bin/bash

cargo build &&
    cargo build --no-default-features &&
    cargo test &&
    cargo test --no-default-features &&
    cargo fmt &&
    cargo clippy