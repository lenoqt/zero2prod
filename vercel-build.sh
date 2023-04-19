#!/bin/bash
yum install clang llvm binutils -y
curl --proto 'https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
echo '[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "linker=clang", "-C", "link-arg=-fuse-ld=ld"]' >> ~/.cargo/config.toml
source "$HOME/.cargo/env"
cargo install cargo-chef
cargo chef prepare --recipe-path recipe.json
cargo chef cook --release --recipe-path recipe.json
SQLX_OFFLINE=true cargo build --release --bin zero2prod
