###########################################################
##### Development
###########################################################

FROM rust:1.49.0-slim-buster AS develop

RUN mkdir /app

WORKDIR /app

RUN apt update && apt install --yes git && rm -rf /var/lib/apt/lists/*

RUN cargo install --git https://github.com/passcod/cargo-watch.git --branch main cargo-watch

###########################################################
##### Builder
###########################################################

FROM develop AS builder

COPY . /app

RUN cargo build --release

###########################################################
##### Release
###########################################################

FROM debian:buster-slim

RUN useradd --create-home loggy

WORKDIR /home/loggy

COPY --from=builder /app/target/release/loggy-server-rs /home/loggy/loggy-server-rs

USER loggy

CMD ["/home/loggy/loggy-rs"]
