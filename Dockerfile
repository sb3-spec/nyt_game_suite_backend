FROM rust:latest

WORKDIR /usr/src/nyt_games_api

COPY . . 

ENV DATABASE_URL=$DATABASE_URL
ENV CACHE_URL=$CACHE_URL


RUN cargo build

EXPOSE 8080

CMD cargo run