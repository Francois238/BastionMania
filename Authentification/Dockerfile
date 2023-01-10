FROM rust:1.64.0 as build

WORKDIR /app
COPY Cargo.* /app/
COPY src/ /app/src

RUN cargo build --release

#FROM alpine:3.17.1
#FROM rust:1.64.0
FROM rust:1.66.0-alpine3.17
COPY --from=build /app/target/release/authentication /
#RUN apk update && apk add postgresql

COPY .env /

EXPOSE 8080

CMD ["./authentication"]