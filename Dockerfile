FROM rust:1.67 AS builder
WORKDIR /usr/src/heiwa
COPY . .
RUN cargo install --path . && cargo install diesel_cli --no-default-features --features postgres

FROM debian:bullseye-slim
ENV DATABASE_URL='postgres://postgres:postgres@db:5432/heiwa'
EXPOSE 8000
COPY --from=builder /usr/src/heiwa/migrations /usr/src/heiwa/migrations
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel
COPY --from=builder /usr/local/cargo/bin/heiwa /usr/local/bin/heiwa
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/* && cd /usr/src/heiwa/ && diesel setup
CMD ["heiwa"]


#FROM lukemathwalker/cargo-chef:latest-rust-1.59.0 AS chef
#WORKDIR heiwa

#FROM chef AS planner
#COPY . .
#RUN cargo chef prepare --recipe-path recipe.json

#FROM chef AS builder
#COPY --from=planner /heiwa/recipe.json recipe.json
#RUN cargo chef cook --release --recipe-path recipe.json && cargo install --path . && cargo install diesel_cli --no-default-features --features postgres
#COPY . .
#RUN cargo build --release --bin heiwa

#FROM debian:bullseye-slim AS runtime
#WORKDIR heiwa
#ENV DATABASE_URL='postgres://postgres:postgres@db:5432/heiwa'
#EXPOSE 8000
#COPY --from=builder /heiwa/migrations /heiwa/migrations
#COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel
#COPY --from=builder /heiwa/target/release/heiwa /usr/local/bin
#RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/* && cd /heiwa && diesel setup
#ENTRYPOINT ["heiwa"]