
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
RUN cargo build --release --target x86_64-unknown-linux-musl

COPY src ./src
RUN cargo build --release --target x86_64-unknown-linux-musl


FROM alpine:3.17

RUN apk add --no-cache --repository http://dl-cdn.alpinelinux.org/alpine/edge/testing wireguard-go \
    && apk add --no-cache wireguard-tools \
    && apk add --no-cache openssh \
    && ssh-keygen -A

EXPOSE ${PORT}
EXPOSE 22/tcp

COPY ./ssh_files/sshd_config /etc/ssh/sshd_config

COPY --from=builder /usr/src/bastion-mania-bastion/target/x86_64-unknown-linux-musl/release/bastion-mania-bastion /app/bastion-mania-bastion
CMD ["/app/bastion-mania-bastion"]
