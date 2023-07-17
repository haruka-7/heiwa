FROM rust:1.67 AS builder
WORKDIR /usr/src/heiwa
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
EXPOSE 8000
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/heiwa /usr/local/bin/heiwa
CMD ["heiwa"]