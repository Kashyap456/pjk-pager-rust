FROM rust:latest as builder

# create a new empty shell project
RUN USER=root cargo new --bin pager-ws
WORKDIR /pager-ws

# copy over your manifests
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/*
RUN cargo build --release

# our final base
FROM ubuntu:20.04

# avoid tzdata interactive dialog
ENV DEBIAN_FRONTEND=noninteractive

# install OpenSSL
RUN apt-get update && apt-get install -y libssl1.1 && rm -rf /var/lib/apt/lists/*

# copy the build artifact from the build stage
COPY --from=builder /pager-ws/target/release/pager-ws .

# set the startup command to run your binary
CMD ["./pager-ws"]