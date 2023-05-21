FROM rust:1.69-slim AS builder

WORKDIR /usr/src
RUN rustup target add x86_64-unknown-linux-musl \
    && cargo new bastion-controller --vcs none \
    && apt update \
    && apt install -y musl-tools \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/bastion-controller

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release --bin bastion-ressource-controller --target x86_64-unknown-linux-musl

COPY src ./src
RUN cargo build --release --bin bastion-ressource-controller --target x86_64-unknown-linux-musl


FROM scratch

COPY --from=builder /usr/src/bastion-controller/target/x86_64-unknown-linux-musl/release/bastion-ressource-controller /app/bastion-ressource-controller
CMD ["/app/bastion-ressource-controller"]
