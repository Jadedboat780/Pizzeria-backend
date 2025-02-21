# Backend for pizzeria
Stack: axum, serde, sqlx, tokio

## Run project
```shell
# Rename .env.example
mv .env.example .env

# Running docker compose
docker compose up -d
```

## Ping server
```shell
curl http://127.0.0.1:3000/ping
```