#!/usr/bin/env bash
# Development: API server on :8090 plus the Dioxus dev server on :8091 (proxies /api).
set -e
cd "$(dirname "$0")"
NRZLEARN_ADDR=127.0.0.1:8090 NRZLEARN_DATA=./data cargo run -p nrzlearn-server &
SERVER=$!
trap 'kill $SERVER 2>/dev/null' EXIT
dx serve --platform web --package nrzlearn --port 8091 "$@"
