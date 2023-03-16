#!/bin/bash
curl https://sh.rustup.rs -sSf | sh -s -- -y

export PATH="$HOME/.cargo/env:$PATH"
source "$HOME/.cargo/env"

rustup toolchain install nightly
rustup default nightly
rustup target add wasm32-unknown-unknown

$HOME/.cargo/bin/cargo install --locked trunk
$HOME/.cargo/bin/trunk build