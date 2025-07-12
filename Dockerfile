FROM rust:1.88 AS builder
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release 

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
RUN useradd -r -s /bin/false app
COPY --from=builder /usr/src/app/target/release/medicate-rust /usr/local/bin/
RUN chown app:app /usr/local/bin/medicate-rust
USER app
EXPOSE 8080
CMD ["medicate-rust"] 