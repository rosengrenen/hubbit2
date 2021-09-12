FROM rust

WORKDIR /usr/src/app

RUN cargo install sqlx-cli

COPY . .

RUN SQLX_OFFLINE=true cargo build --release

ENV PORT=8080
EXPOSE 8080

CMD cargo sqlx migrate run && cargo run --release
