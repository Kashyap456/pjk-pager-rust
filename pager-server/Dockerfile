FROM rust:latest as builder

# create a new empty shell project
RUN USER=root cargo new --bin pager-server
WORKDIR /pager-server

# copy over your manifests
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src
COPY ./migrations ./migrations
COPY ./sqlite.db ./sqlite.db
COPY ./sqlx-data.json ./sqlx-data.json  

# build for release
RUN rm ./target/release/deps/*
ENV DATABASE_URL=sqlite://sqlite.db
RUN ln -s ./sqlite.db ../sqlite.db
RUN cargo build --release

# our final base
FROM rust:latest

# install OpenSSL
RUN apt-get update && apt-get install -y libssl1.1 && rm -rf /var/lib/apt/lists/*

# copy the build artifact from the build stage
COPY --from=builder /pager-server/target/release/pager-server .
# copy the migrations directory from the build stage
COPY --from=builder /pager-server/migrations ./migrations

# set the startup command to run your binary
CMD ["./pager-server"]