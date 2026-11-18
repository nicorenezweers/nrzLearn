#!/usr/bin/env bash
# Builds the complete web app into target/dx/nrzlearn/release/web/public:
# the WebAssembly app (dx), the static pages, course texts, sitemap … (nrzlearn-site)
# and pre-compressed copies (.br, .gz) of every text file.
#
#   ./scripts/build-web.sh [--base-url https://learn.example.org]
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${ROOT}"
export RUSTUP_TOOLCHAIN="${RUSTUP_TOOLCHAIN:-stable}"

# dioxus-cli finden (`dx` im PATH kann auch Denos dx sein).
DX="${DX:-}"
for c in "${DX}" "$(command -v dx || true)" "${HOME}/.cargo/bin/dx" "$(dirname "$(command -v cargo)")/dx"; do
  if [ -n "${c}" ] && "${c}" --version 2>/dev/null | grep -qi dioxus; then DX="${c}"; break; fi
  DX=""
done
[ -n "${DX}" ] || { echo "dioxus-cli (dx) not found - cargo binstall dioxus-cli@0.7.10" >&2; exit 1; }

WEB="target/dx/nrzlearn/release/web/public"
# dx does not clean up old files; stale assets would end up in the package.
rm -rf target/dx/nrzlearn/release
"${DX}" bundle --platform web --release --package nrzlearn
cargo run -q --release -p nrzlearn-site -- build "${WEB}" "$@"

if command -v brotli >/dev/null; then BR=1; else BR=0; echo "brotli not installed: only gzip" >&2; fi
find "${WEB}" -type f \( -name '*.html' -o -name '*.css' -o -name '*.js' -o -name '*.wasm' -o -name '*.svg' -o -name '*.json' -o -name '*.xml' -o -name '*.txt' -o -name '*.webmanifest' \) -size +1k -print0 |
  while IFS= read -r -d '' f; do
    gzip -9 -k -f "$f"
    if [ "${BR}" = 1 ]; then brotli -q 11 -k -f "$f"; fi
  done
echo "==> ${WEB} ($(du -sh "${WEB}" | cut -f1))"
