
ARG APP_NAME="bastion-mania-agent"
ARG PORT=60469

FROM rust:1.66-slim AS builder

WORKDIR /usr/src
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo new bastion-mania-agent --vcs none
WORKDIR /usr/src/bastion-mania-agent

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .


FROM alpine:3.17

RUN apk add -U wireguard-tools

EXPOSE ${PORT}

COPY --from=builder /usr/local/cargo/bin/bastion-mania-agent /app/bastion-mania-agent
CMD ["/app/bastion-mania-agent"]
