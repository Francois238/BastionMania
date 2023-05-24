
FROM rust:1.69-slim AS builder

WORKDIR /usr/src
RUN cargo new bastion-mania-bastioninstancieur --vcs none \
    && apt-get update \
    && apt-get install -y libssl-dev pkg-config\
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/bastion-mania-bastioninstancieur


COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN cargo build --release


FROM gcr.io/distroless/cc

EXPOSE 9000

USER 1000

COPY --from=builder /usr/src/bastion-mania-bastioninstancieur/target/release/bastion-mania-bastioninstancieur /app/bastion-mania-bastioninstancieur
CMD ["/app/bastion-mania-bastioninstancieur"]
