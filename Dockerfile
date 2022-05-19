FROM rust:latest AS chef
RUN cargo install cargo-chef
WORKDIR /src/app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /src/app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
# building app
COPY src src
RUN cargo build --release
RUN ls -la ./target/release/

FROM rust:1.60-slim AS runtime

WORKDIR /src/app

COPY --from=builder /src/app/target/release/mysql_async_bug /usr/local/bin

EXPOSE 8080

CMD ["mysql_async_bug"]

