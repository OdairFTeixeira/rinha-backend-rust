FROM rust:latest as builder
WORKDIR /usr/src/rinha-backend-rust
COPY . .
RUN cargo build --release
FROM ubuntu:22.04
WORKDIR /usr/src/rinha-backend-rust
COPY --from=builder /usr/src/rinha-backend-rust/target/release/rinha-backend-rust .
RUN apt-get update && apt-get install -y \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*
EXPOSE 3333
CMD ["./rinha-backend-rust"]
