FROM rust:1.67 AS builder
WORKDIR /usr/src/heiwa
COPY . .
RUN echo "DATABASE_URL=postgres://postgres:postgres@postgres:5432/heiwa" > .env && cargo install --path . && cargo install diesel_cli --no-default-features --features postgres && diesel setup

FROM debian:bullseye-slim
EXPOSE 8000
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/heiwa /usr/local/bin/heiwa
CMD ["heiwa"]