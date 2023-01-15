
ARG PORT=60244

FROM rust:1.66-slim AS builder

WORKDIR /usr/src
RUN rustup target add x86_64-unknown-linux-musl \
    && cargo new bastion-mania-bastion --vcs none \
    && apt update \
    && apt install -y musl-tools \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/bastion-mania-bastion

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .


FROM alpine:3.17

RUN apk add --no-cache --repository http://dl-cdn.alpinelinux.org/alpine/edge/testing wireguard-go \
    && apk add -U wireguard-tools

EXPOSE ${PORT}

COPY --from=builder /usr/local/cargo/bin/bastion-mania-bastion /app/bastion-mania-bastion
CMD ["/app/bastion-mania-bastion"]
