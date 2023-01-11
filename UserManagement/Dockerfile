FROM rust:1.64.0 as build

WORKDIR /app
COPY Cargo.* /app/
COPY src/ /app/src

RUN cargo build --release


FROM rust:1.64.0
COPY --from=build /app/target/release/user_management /


EXPOSE 8082

CMD ["/user_management"]