
ARG PORT=60469

FROM rust:1.69-slim AS builder

WORKDIR /usr/src
RUN cargo new bastion-mania-agent --vcs none \
    && apt update  \
    && apt install libssl-dev pkg-config -y  \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/bastion-mania-agent

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN cargo build --release


FROM debian:11.7-slim

RUN apt update && apt install wireguard-tools -y && rm -rf /var/lib/apt/lists/*

EXPOSE ${PORT}

COPY --from=builder /usr/src/bastion-mania-agent/target/release/bastion-mania-agent /app/bastion-mania-agent
CMD ["/app/bastion-mania-agent"]
