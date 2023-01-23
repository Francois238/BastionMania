FROM rust:1.64.0 as build

WORKDIR /app
COPY Cargo.* /app/
COPY src/ /app/src

RUN cargo build --release

FROM ubuntu:23.04

WORKDIR /app
COPY --from=build /app/target/release/BastionManager /app/BastionManager

RUN  apt-get update \
  && apt-get install -y libpq-dev openssl wget\
    && wget http://nz2.archive.ubuntu.com/ubuntu/pool/main/o/openssl/libssl1.1_1.1.1f-1ubuntu2.16_amd64.deb \
    && dpkg -i libssl1.1_1.1.1f-1ubuntu2.16_amd64.deb \
  && rm -rf /var/lib/apt/lists/*

EXPOSE 8080

CMD ["/app/BastionManager"]