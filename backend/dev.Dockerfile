FROM rust

WORKDIR /usr/src/app

RUN cargo install sqlx-cli cargo-watch

ENV PORT=8080
EXPOSE 8080

CMD cargo sqlx migrate run && cargo watch -x run
