FROM rust:1.64.0 as build

WORKDIR /app
COPY Cargo.* /app/
COPY src/ /app/src

RUN cargo build --release

FROM ubuntu:23.04

WORKDIR /app
COPY --from=build /app/target/release/admin_management /app/admin_management

RUN  apt-get update \
  && apt-get install -y wget \
  && rm -rf /var/lib/apt/lists/*

RUN apt update
RUN apt install -y libpq-dev

EXPOSE 8081

CMD ["/app/admin_management"]