#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- -- "$0")" && pwd)"
cd "$SCRIPT_DIR"

RUST_TOOLCHAIN="1.100.0"
TRUNK_VERSION="0.22.1"
TRUNK_SHA256="2ab8d548fd5ed9cda50906f61fd8654204e7893bd52ebbfd90533862b7d71ee1"
ARCHIVE="$RUNNER_TEMP/trunk-x86_64-unknown-linux-gnu.tar.gz"

if [[ -z "${RUNNER_TEMP:-}" ]]; then
  ARCHIVE="${TMPDIR:-/tmp}/trunk-x86_64-unknown-linux-gnu.tar.gz"
fi

if ! command -v rustup >/dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 --retry 3 --retry-connrefused -fsSL \
    https://sh.rustup.rs | sh -s -- -y --profile minimal
fi
export PATH="$HOME/.cargo/bin:$PATH"

rustup toolchain install "$RUST_TOOLCHAIN" --profile minimal \
  --target wasm32-unknown-unknown
export RUSTUP_TOOLCHAIN="$RUST_TOOLCHAIN"

mkdir -p "$HOME/.cargo/bin" "${TMPDIR:-/tmp}/sherlock-trunk"
curl --fail --location --retry 3 --silent --show-error \
  "https://github.com/trunk-rs/trunk/releases/download/v${TRUNK_VERSION}/trunk-x86_64-unknown-linux-gnu.tar.gz" \
  --output "$ARCHIVE"
echo "$TRUNK_SHA256  $ARCHIVE" | sha256sum --check
tar -xzf "$ARCHIVE" -C "${TMPDIR:-/tmp}/sherlock-trunk"
TRUNK_BINARY="$(find "${TMPDIR:-/tmp}/sherlock-trunk" -type f -name trunk -perm -111 -print -quit)"
test -n "$TRUNK_BINARY"
install "$TRUNK_BINARY" "$HOME/.cargo/bin/trunk"

rustc --version
cargo --version
trunk --version
trunk build --release

test -s dist/index.html
WASM_FILE="$(find dist -maxdepth 1 -type f -name '*.wasm' -size +0c -print -quit)"
JS_FILE="$(find dist -maxdepth 1 -type f -name '*.js' -size +0c -print -quit)"
test -n "$WASM_FILE"
test -n "$JS_FILE"

echo "Verified browser bundle:"
ls -lh dist/index.html "$WASM_FILE" "$JS_FILE"
