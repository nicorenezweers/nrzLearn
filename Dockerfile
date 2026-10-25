# syntax=docker/dockerfile:1

# ── Web app (WebAssembly is the same for every platform, so build it natively) ──
FROM --platform=$BUILDPLATFORM rust:1-trixie AS web
RUN rustup target add wasm32-unknown-unknown \
 && curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash \
 && cargo binstall -y --locked dioxus-cli@0.7.10
WORKDIR /src
COPY . .
RUN dx bundle --platform web --release --package nrzlearn \
 && cp -r target/dx/nrzlearn/release/web/public /public

# ── Server ──
FROM rust:1-trixie AS server
WORKDIR /src
COPY . .
RUN cargo build --release -p nrzlearn-server

# ── Runtime ──
FROM debian:trixie-slim
RUN useradd --system --uid 10001 --home-dir /app nrzlearn \
 && mkdir -p /data \
 && chown nrzlearn /data
COPY --from=server /src/target/release/nrzlearn-server /usr/local/bin/nrzlearn-server
COPY --from=web /public /app/public
ENV NRZLEARN_ADDR=0.0.0.0:8080 \
    NRZLEARN_DATA=/data \
    NRZLEARN_PUBLIC=/app/public
VOLUME /data
EXPOSE 8080
USER nrzlearn
CMD ["nrzlearn-server"]
