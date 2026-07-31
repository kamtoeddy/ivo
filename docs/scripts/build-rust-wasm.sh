#!/usr/bin/env bash
# Builds wasm/ivo-playground/ (a small crate wrapping the ivo Rust crate with
# wasm-bindgen exports for a curated set of demos - see docs/README.md for why
# this is bounded demos rather than arbitrary code execution) and emits the JS
# bindings + .wasm binary into static/wasm/ivo-playground/, where
# src/components/RustPlayground loads them client-side.
#
# Requires: rustup target add wasm32-unknown-unknown, and wasm-pack
# (https://rustwasm.github.io/wasm-pack/installer/).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DOCS_ROOT="$(dirname "$SCRIPT_DIR")"
CRATE_DIR="$DOCS_ROOT/wasm/ivo-playground"
OUT_DIR="$DOCS_ROOT/static/wasm/ivo-playground"

if ! command -v wasm-pack >/dev/null 2>&1; then
  echo "wasm-pack not found - install it: https://rustwasm.github.io/wasm-pack/installer/" >&2
  exit 1
fi

if ! rustup target list --installed 2>/dev/null | grep -q wasm32-unknown-unknown; then
  echo "wasm32-unknown-unknown target not installed - run: rustup target add wasm32-unknown-unknown" >&2
  exit 1
fi

cd "$CRATE_DIR"
wasm-pack build --target web --out-dir ../../static/wasm/ivo-playground

echo "Built wasm/ivo-playground -> $OUT_DIR"
