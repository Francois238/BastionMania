FROM rust:1.64.0 as build

WORKDIR /app
COPY Cargo.* /app/
COPY src/ /app/src

COPY .env /app/

RUN cargo build --release


#FROM gcr.io/distroless/cc
#COPY --from=build /app/target/release/admin_management /
#COPY --from=build /app/.env /
#COPY .env /

EXPOSE 8081

CMD ["/app/target/release/admin_management"]