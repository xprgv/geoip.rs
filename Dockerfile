FROM rust:1.63.0-alpine3.16 AS builder
WORKDIR /artifacts
COPY ./Cargo.lock ./
COPY ./Cargo.toml ./
COPY ./src ./src
# RUN ls -l
COPY . .
# RUN cargo build --release
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM --platform=linux/amd64 alpine:3.16
# RUN --platform=linux/amd64 debian:buster-slim
WORKDIR /app
COPY --from=builder /artifacts/target/release/geoip .
COPY ./assets/GeoLite2-City.mmdb /app/GeoLite2-City.mmdb
CMD ["./geoip", "--mmdb", "./GeoLite2-City.mmdb"]