FROM rust:latest

WORKDIR /src/app

COPY Cargo.lock Cargo.toml ./
# RUN cargo fetch

COPY src src
RUN cargo install --path .

EXPOSE 8080

CMD ["mysql_async_bug"]

