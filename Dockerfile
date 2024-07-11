FROM rust:latest

WORKDIR /usr/src/nyt_games_api

# ARG DATABASE_URL=postgresql://postgres:qyFQOjqgeCSGQjexvUcMjtnuDfJoXkbu@viaduct.proxy.rlwy.net:26899/railway
# ENV DATABASE_URL=postgresql://postgres:qyFQOjqgeCSGQjexvUcMjtnuDfJoXkbu@viaduct.proxy.rlwy.net:26899/railway

# ARG CACHE_URL=redis://default:wcLiJFeJgAlIVHymcOAQdSSStCQiLDKg@monorail.proxy.rlwy.net:10481
# ENV CACHE_URL=redis://default:wcLiJFeJgAlIVHymcOAQdSSStCQiLDKg@monorail.proxy.rlwy.net:10481

COPY . . 

RUN cargo build

EXPOSE 8080

CMD cargo run