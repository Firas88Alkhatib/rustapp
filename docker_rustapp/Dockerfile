FROM rust:1.64
RUN apt update
RUN apt install libpq-dev
RUN cargo install diesel_cli --no-default-features --features postgres
# cargo watch is very slow in docker
# RUN cargo install cargo-watch