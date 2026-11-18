#!/usr/bin/env bash
# Baut nrzLearn fuer umbrelOS (Linux amd64 + arm64), legt die fertigen Dateien auf den Zweig
# `umbrel` und pusht den Tag `umbrel-<version>`. nrzGit traegt die Version dann per nrzStore.json
# selbst im nrzStore ein (RELEASE in hooks/pre-start, version, releaseNotes, Store-Version).
#
#   ./scripts/umbrel-release.sh 4.0.1 "Kurze Beschreibung"
#
# Mit --no-push wird nur gebaut und der Tag lokal angelegt (zum Pruefen).
#
# Voraussetzungen: dx (dioxus-cli 0.7.10), zig + cargo-zigbuild (Linker fuer Linux), brotli,
# Rust-Targets wasm32-unknown-unknown, x86_64-unknown-linux-musl und aarch64-unknown-linux-musl.
# Die Web-App baut scripts/build-web.sh (App, statische SEO-Seiten, Kurstexte, Vorkomprimierung).
set -euo pipefail

PUSH=1
if [ "${1:-}" = "--no-push" ]; then PUSH=0; shift; fi
VERSION="${1:?Version fehlt, z.B. 4.0.1}"
NOTES="${2:-nrzLearn ${VERSION}}"
TAG="umbrel-${VERSION}"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${ROOT}"
# Ohne rustup-Standard (z. B. frische Umgebung) stable nehmen.
export RUSTUP_TOOLCHAIN="${RUSTUP_TOOLCHAIN:-stable}"

if [ -n "$(git status --porcelain)" ]; then
  echo "Arbeitsverzeichnis nicht sauber - erst committen." >&2
  exit 1
fi
if git rev-parse -q --verify "refs/tags/${TAG}" >/dev/null; then
  echo "Tag ${TAG} gibt es schon." >&2
  exit 1
fi
CARGO_VERSION="$(sed -n 's/^version = "\(.*\)"$/\1/p' Cargo.toml | head -1)"
if [ "${CARGO_VERSION}" != "${VERSION}" ]; then
  echo "Cargo.toml hat Version ${CARGO_VERSION}, nicht ${VERSION}." >&2
  exit 1
fi

# Kurse, Uebersetzungen und Oberflaechentexte muessen vollstaendig und gueltig sein.
# NRZLEARN_ALLOW_MISSING=1: noch nicht uebersetzte Oberflaechentexte sind nur Warnungen
# (die App und die Seiten nutzen dann Englisch), z. B. waehrend neue Uebersetzungen laufen.
CHECK_ARGS=()
if [ "${NRZLEARN_ALLOW_MISSING:-0}" = 1 ]; then CHECK_ARGS=(--allow-missing); fi
cargo run -q -p nrzlearn-site -- check-all ${CHECK_ARGS[@]+"${CHECK_ARGS[@]}"} >/dev/null || { echo "nrzlearn-site check-all meldet Fehler - Abbruch." >&2; exit 1; }
cargo zigbuild --help >/dev/null 2>&1 || { echo "cargo-zigbuild fehlt (cargo install cargo-zigbuild; brew install zig)" >&2; exit 1; }
command -v brotli >/dev/null || { echo "brotli fehlt (brew install brotli)" >&2; exit 1; }

OUT="$(mktemp -d)"
trap 'rm -rf "${OUT}"' EXIT
WEB="target/dx/nrzlearn/release/web/public"

echo "==> Web-App und statische Seiten (einmal fuer alle Architekturen)"
./scripts/build-web.sh
[ -f "${WEB}/app.html" ] && [ -f "${WEB}/index.html" ] || { echo "Build unvollstaendig - Abbruch." >&2; exit 1; }
# Seiten, Kurstexte und Sitemap ohne ihre vorkomprimierten Kopien (.br/.gz): die machen das Paket
# mehrfach groesser. hooks/pre-start erzeugt die gzip-Kopien auf dem Geraet. App, CSS und Logos
# bleiben vorkomprimiert (Brotli).
COPYFILE_DISABLE=1 tar --no-xattrs \
  --exclude='*.html.br' --exclude='*.html.gz' --exclude='*.json.br' --exclude='*.json.gz' \
  --exclude='*.xml.br' --exclude='*.xml.gz' --exclude='*.txt.br' --exclude='*.txt.gz' \
  -czf "${OUT}/public.tar.gz" -C "${WEB}" .

for pair in "x86_64-unknown-linux-musl amd64" "aarch64-unknown-linux-musl arm64"; do
  triple="${pair% *}"
  arch="${pair#* }"
  echo "==> Server ${arch} (${triple}, statisch)"
  cargo zigbuild --release -p nrzlearn-server --target "${triple}"
  gzip -9 -c "target/${triple}/release/nrzlearn-server" > "${OUT}/nrzlearn-server-${arch}.gz"
done
(cd "${OUT}" && shasum -a 256 nrzlearn-server-amd64.gz nrzlearn-server-arm64.gz public.tar.gz > SHA256SUMS)

# Zweig `umbrel` in einem eigenen Worktree aktualisieren (nur fertige Dateien).
WT="$(mktemp -d)"
git fetch -q origin umbrel 2>/dev/null || true
if git rev-parse -q --verify origin/umbrel >/dev/null; then
  git worktree add -q "${WT}" -B umbrel origin/umbrel
else
  git worktree add -q --detach "${WT}"
  (cd "${WT}" && git checkout -q --orphan umbrel && git rm -rqf .)
fi
cp "${OUT}"/* "${WT}/"
git show HEAD:scripts/umbrel-README.md > "${WT}/README.md"
(
  cd "${WT}"
  git add -A
  git commit -qm "nrzLearn ${VERSION}: Umbrel-Paket"
  git tag -a "${TAG}" -m "${NOTES}"
  if [ "${PUSH}" = 1 ]; then
    git push -q origin umbrel
    git push -q origin "${TAG}"
  fi
)
git worktree remove --force "${WT}"
if [ "${PUSH}" = 1 ]; then
  echo "==> ${TAG} veroeffentlicht. nrzGit aktualisiert den nrzStore."
else
  echo "==> ${TAG} lokal angelegt (Zweig umbrel), nichts gepusht."
fi
