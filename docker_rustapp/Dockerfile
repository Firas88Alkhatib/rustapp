FROM rust:1.64
RUN apt update
RUN apt install libpq-dev
RUN cargo install diesel_cli --no-default-features --features postgres