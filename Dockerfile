FROM rust:1.63.0-alpine3.16 AS builder
WORKDIR /artifacts
RUN apk add --no-cache libc-dev=0.7.2-r3
COPY ./Cargo.lock ./
COPY ./Cargo.toml ./
COPY ./src ./src
RUN cargo build --release

FROM --platform=linux/amd64 alpine:3.16
WORKDIR /app
COPY --from=builder /artifacts/target/release/geoip .
COPY ./assets/GeoLite2-City.mmdb /app/GeoLite2-City.mmdb
RUN ls -l
CMD ["./geoip", "--addr", "127.0.0.1:3000", "--mmdb-path", "./GeoLite2-City.mmdb"]