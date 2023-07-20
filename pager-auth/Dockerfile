FROM rust:latest as builder

# create a new empty shell project
RUN USER=root cargo new --bin pager-auth
WORKDIR /pager-auth

# copy over your manifests
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src
COPY ./migrations ./migrations
COPY ./keyvalue.db ./keyvalue.db
COPY ./sqlx-data.json ./sqlx-data.json  

# build for release
RUN rm ./target/release/deps/*
ENV DATABASE_URL=sqlite://keyvalue.db
RUN ln -s ./keyvalue.db ../keyvalue.db
RUN ls -al
RUN cargo build

# our final base
FROM rust:latest

# install OpenSSL
RUN apt-get update && apt-get install -y libssl1.1 && rm -rf /var/lib/apt/lists/*

# copy the build artifact from the build stage
COPY --from=builder /pager-auth/target/debug/pager-auth .
# copy the migrations directory from the build stage
COPY --from=builder /pager-auth/migrations ./migrations

# set the startup command to run your binary
CMD ["./pager-auth"]