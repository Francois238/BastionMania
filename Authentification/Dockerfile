##### Builder
FROM rust:1.69 as builder

WORKDIR /usr/src

# Create blank project
RUN USER=root cargo new authentication

# We want dependencies cached, so copy those first.
COPY Cargo.toml Cargo.lock /usr/src/authentication/

# Set the working directory
WORKDIR /usr/src/authentication

## Install target platform (Cross-Compilation) --> Needed for Alpine
RUN rustup target add x86_64-unknown-linux-musl

# This is a dummy build to get the dependencies cached.
RUN cargo build --target x86_64-unknown-linux-musl --release

# Now copy in the rest of the sources
COPY src /usr/src/authentication/src/

## Touch main.rs to prevent cached release build
RUN touch /usr/src/authentication/src/main.rs

# This is the actual application build.
RUN cargo build --target x86_64-unknown-linux-musl --release

################
##### Runtime
FROM alpine:3.17.3 AS runtime 

# Copy application binary from builder image
COPY --from=builder /usr/src/authentication/target/x86_64-unknown-linux-musl/release/authentication /usr/local/bin

EXPOSE 8080

# Run the application
CMD ["/usr/local/bin/authentication"]
