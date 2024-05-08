FROM rust:latest
WORKDIR /app
COPY . /app
CMD ["cargo", "run", "--release"]






