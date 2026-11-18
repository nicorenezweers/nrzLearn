#!/usr/bin/env bash
# Development: the Dioxus dev server on :8091 with hot reload. nrzLearn needs
# no backend. The course texts are written to public/content first; the
# static landing pages only exist in a full build (scripts/build-web.sh).
set -e
cd "$(dirname "$0")"
export RUSTUP_TOOLCHAIN="${RUSTUP_TOOLCHAIN:-stable}"
cargo run -q -p nrzlearn-site -- dev-content
# `dx` in PATH can also be Deno's dx; prefer the dioxus-cli from cargo.
DX=dx
if ! dx --version 2>/dev/null | grep -qi dioxus && [ -x "$HOME/.cargo/bin/dx" ]; then DX="$HOME/.cargo/bin/dx"; fi
exec "$DX" serve --platform web --package nrzlearn --port 8091 "$@"
