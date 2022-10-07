FROM rust:1.64.0-alpine3.16 AS builder
WORKDIR /build
RUN apk add --no-cache libc-dev=0.7.2-r3
# caching dependencies
COPY Cargo.* /build/
RUN mkdir /build/src && echo "fn main() {}" > /build/src/main.rs
RUN cargo build --release
# build project
COPY ./src ./src
RUN touch src/main.rs && cargo build --release

FROM --platform=linux/amd64 alpine:3.16
WORKDIR /app
COPY --from=builder /build/target/release/geoip .
COPY ./assets/GeoLite2-City.mmdb /app/GeoLite2-City.mmdb
CMD ["./geoip", "--addr", "127.0.0.1:3000", "--mmdb-path", "./GeoLite2-City.mmdb"]