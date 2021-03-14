FROM rust:1.50

WORKDIR /usr/src/app

RUN cargo install sqlx-cli

COPY migrations migrations
COPY src src
COPY Cargo.toml Cargo.lock ./
COPY sqlx-data.json sqlx-data.json

RUN cargo build --release

CMD cargo sqlx migrate run && cargo run --release
