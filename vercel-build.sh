curl --proto 'https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
apt update && apt install lld clang -y
cargo build --release
