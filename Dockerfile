FROM rust:latest

WORKDIR /usr/src/nyt_games_api

COPY . . 

RUN cargo build

EXPOSE 8080

CMD cargo run