FROM rust:1.67 AS builder
WORKDIR /usr/src/heiwa
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
EXPOSE 8000
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
RUN cargo install diesel_cli --no-default-features --features postgres && diesel setup
COPY --from=builder /usr/src/heiwa/migrations /usr/src/heiwa/
COPY --from=builder /usr/local/cargo/bin/heiwa /usr/local/bin/heiwa
CMD ["cd /usr/src/heiwa/ && diesel setup && heiwa"]