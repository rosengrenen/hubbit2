FROM rust:1.50

WORKDIR /usr/src/app

RUN cargo install sqlx-cli

COPY . .

RUN SQLX_OFFLINE=true cargo build --release

ENV PORT=3000

CMD cargo sqlx migrate run && cargo run --release
