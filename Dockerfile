# syntax=docker/dockerfile:1

# ── Web app (WebAssembly is the same for every platform, so build it natively) ──
FROM --platform=$BUILDPLATFORM rust:1-trixie AS web
ARG NRZLEARN_BASE_URL=https://learn.zweers.ch
ENV NRZLEARN_BASE_URL=${NRZLEARN_BASE_URL}
RUN rustup target add wasm32-unknown-unknown \
 && curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash \
 && cargo binstall -y --locked dioxus-cli@0.7.10 \
 && apt-get update && apt-get install -y --no-install-recommends brotli && rm -rf /var/lib/apt/lists/*
WORKDIR /src
COPY . .
# App, static pages for search engines, course texts, then pre-compress once,
# so the server never compresses per request.
RUN dx bundle --platform web --release --package nrzlearn \
 && cargo run --release -p nrzlearn-site -- build target/dx/nrzlearn/release/web/public \
 && cp -r target/dx/nrzlearn/release/web/public /public \
 && find /public -type f \( -name '*.html' -o -name '*.css' -o -name '*.js' -o -name '*.wasm' -o -name '*.svg' -o -name '*.json' -o -name '*.xml' -o -name '*.txt' -o -name '*.webmanifest' \) -size +1k \
      -exec brotli -q 11 -k {} \; -exec gzip -9 -k {} \;

# ── Server: only hands out the static files ──
FROM rust:1-trixie AS server
WORKDIR /src
COPY . .
RUN cargo build --release -p nrzlearn-server

# ── Runtime: no database; /data only keeps the admin settings and the anonymous statistics ──
# Static pages use https://learn.zweers.ch for canonical URLs; build with
# --build-arg / NRZLEARN_BASE_URL in the web stage for another domain.
FROM debian:trixie-slim
RUN useradd --system --uid 10001 --home-dir /app nrzlearn && mkdir /data && chown nrzlearn /data
COPY --from=server /src/target/release/nrzlearn-server /usr/local/bin/nrzlearn-server
COPY --from=web /public /app/public
ENV NRZLEARN_ADDR=0.0.0.0:8080 \
    NRZLEARN_PUBLIC=/app/public \
    NRZLEARN_DATA=/data \
    NRZLEARN_ADMIN_ADDR=0.0.0.0:8081
# 8080: the app. 8081: admin area (only with NRZLEARN_ADMIN_PASSWORD set).
EXPOSE 8080 8081
VOLUME /data
USER nrzlearn
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s CMD ["nrzlearn-server", "healthcheck"]
CMD ["nrzlearn-server"]
