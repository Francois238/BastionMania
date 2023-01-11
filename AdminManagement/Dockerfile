FROM rust:1.64.0 as build

WORKDIR /app
COPY Cargo.* /app/
COPY src/ /app/src

RUN cargo build --release


FROM rust:1.64.0
COPY --from=build /app/target/release/admin_management /


EXPOSE 8081

CMD ["/admin_management"]