FROM rust:1.88-trixie AS builder

WORKDIR /app
RUN cargo install dioxus-cli --version 0.7.10 --locked

COPY . .
RUN mkdir -p target/dx/client/release/web/public/assets && \
    dx build --release --platform web --package client

FROM caddy:2-alpine

COPY client/Caddyfile /etc/caddy/Caddyfile
COPY --from=builder /app/target/dx/client/release/web/public /srv

EXPOSE 8080
