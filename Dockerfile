FROM rust:latest

WORKDIR /usr/src/nyt_games_api

ARG DATABASE_URL
ARG CACHE_URL

ENV DATABASE_URL = ${DATABASE_URL}
ENV CACHE_URL = ${CACHE_URL}

COPY . . 

RUN cargo build

EXPOSE 8080

CMD cargo run