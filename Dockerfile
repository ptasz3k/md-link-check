# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build

RUN apt-get update

RUN apt-get install libssl-dev -y

WORKDIR /usr/src/md-link-check

COPY . .

RUN cargo build --release 

RUN rm -f target/release/deps/md-link-check*


# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM ubuntu:focal

RUN apt-get update
RUN apt-get install openssl -y

WORKDIR /usr/bin/

COPY --from=cargo-build /usr/src/md-link-check/target/release/md-link-check .

