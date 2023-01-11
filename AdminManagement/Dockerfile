FROM rust:1.64.0 as build

WORKDIR /app
COPY Cargo.* /app/
COPY src/ /app/src

RUN cargo build --release

FROM ubuntu:23.04

COPY --from=build /app/target/release/admin_management /app/admin_management

WORKDIR /tmp

RUN  apt-get update  \
  && apt-get install -y libpq-dev \
  && rm -rf /var/lib/apt/lists/* \

RUN wget http://nz2.archive.ubuntu.com/ubuntu/pool/main/o/openssl/libssl1.1_1.1.1f-1ubuntu2.16_amd64.deb \
    && dpkg -i ./libssl1.1_1.1.1f-1ubuntu2.16_amd64.deb


EXPOSE 8081

CMD ["/app/admin_management"]