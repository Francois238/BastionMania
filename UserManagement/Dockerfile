FROM rust:1.64.0 as build

WORKDIR /app
COPY Cargo.* /app/
COPY src/ /app/src

RUN cargo build --release

FROM ubuntu:23.04

WORKDIR /app
COPY --from=build /app/target/release/user_management /app/user_management

RUN  apt-get update \
  && apt-get install -y wget \
  && rm -rf /var/lib/apt/lists/*

RUN apt update
RUN apt install -y libpq-dev

EXPOSE 8082

CMD ["/app/user_management"]