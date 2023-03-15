#!/bin/sh
curl https://sh.rustup.rs -sSf | sh -s -- -y
export PATH="$HOME/.cargo/env:$PATH"
cargo install --locked trunk
trunk build