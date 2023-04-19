#!/bin/bash
yum install clang llvm -y
curl --proto 'https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
cargo build --release
