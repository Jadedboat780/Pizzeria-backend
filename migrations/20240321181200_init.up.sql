CREATE TABLE IF NOT EXISTS users
(
    id         SERIAL PRIMARY KEY,
    username   VARCHAR(40) NOT NULL,
    email      VARCHAR(80) NOT NULL UNIQUE,
    password   VARCHAR(100) NOT NULL,
    address    VARCHAR(100),
    phone      VARCHAR(12),
    avatar_url VARCHAR(100)
);
