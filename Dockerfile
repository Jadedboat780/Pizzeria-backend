FROM rust:1.81-slim AS builder
RUN apt-get update && apt-get install musl-tools -y && rustup target add x86_64-unknown-linux-musl
WORKDIR /usr/src/app
COPY . .
EXPOSE 3000
CMD ["cargo", "run", "--release", "--target=x86_64-unknown-linux-musl"]






