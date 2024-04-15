FROM rust
WORKDIR /app
COPY . /app
#RUN cargo install sqlx-cli && echo .env && sqlx migrate run



