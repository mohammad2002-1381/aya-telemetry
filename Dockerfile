FROM rust:1.93-slim-bookworm AS builder
RUN apt-get update && apt-get install -y \
    clang \
    llvm \
    libelf-dev \
    build-essential \
    musl-tools \
    && rm -rf /var/lib/apt/lists/*
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/app

COPY . .


RUN cargo test --release

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch

WORKDIR /

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/aya-telemetry /telemetry-app

CMD ["/telemetry-app"]
