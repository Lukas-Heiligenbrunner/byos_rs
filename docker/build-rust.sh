#!/usr/bin/env bash
# this script takes two arguments and sets up unattended AUR access for user ${1} via a helper, ${2}
set -o pipefail
set -o errexit
set -o nounset
set -o verbose
set -o xtrace

TARGET_ARCH="${1:-ab}"

if [ "$TARGET_ARCH" == "linux/arm64/v8" ]; then
    rustup target add aarch64-unknown-linux-gnu
    apt update -y && apt install -y gcc-aarch64-linux-gnu
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc cargo build --release --target=aarch64-unknown-linux-gnu
    mv target/aarch64-unknown-linux-gnu/release/byos_rs target/byos_rs
else
  cargo build --release
  mv target/release/byos_rs target/byos_rs
fi