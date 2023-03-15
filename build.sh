#!/bin/sh
curl https://sh.rustup.rs -sSf | sh -s -- -y
export PATH="$HOME/.cargo/env:$PATH"
$HOME/.cargo/bin/cargo install --locked trunk
$HOME/.cargo/bin/trunk build