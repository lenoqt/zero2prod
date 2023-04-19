#!/bin/bash
yum update
yum install clang lld -y
curl --proto 'https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
cargo build --release
