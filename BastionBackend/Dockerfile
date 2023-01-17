FROM rust:1.64.0 as build

WORKDIR /app
COPY Cargo.* /app/
COPY src/ /app/src

RUN cargo build --release

FROM ubuntu:23.04

WORKDIR /app
COPY --from=build /app/target/release/bastionbackend /app/bastionbackend

RUN  apt-get update \
  && apt-get install -y libpq-dev \
  && rm -rf /var/lib/apt/lists/*

EXPOSE 8080

CMD ["/app/authentication"]