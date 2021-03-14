FROM rust:1.50

WORKDIR /usr/src/app

RUN cargo install sqlx-cli cargo-watch

CMD cargo watch -x 'sqlx migrate run' -x run
