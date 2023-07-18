# Use an official Rust runtime as a parent image
FROM rust:latest as builder

# Set the working directory in the builder image
WORKDIR /usr/src

# Copy over your manifest
COPY ./Cargo.toml ./Cargo.toml

# Build dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code into the container
COPY ./src ./src

# Build the application
RUN rm ./target/release/deps/pager-auth*
RUN cargo build --release

# Our final image
FROM debian:buster-slim

# Copy the build artifact from the builder stage and set the working directory
WORKDIR /usr/local/bin
COPY --from=builder /usr/src/target/release/pager-auth .

# Set the startup command to run your binary
CMD ["./pager-auth"]
