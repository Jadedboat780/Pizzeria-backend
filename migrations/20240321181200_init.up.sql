CREATE TABLE IF NOT EXISTS Users
(
    id         SERIAL PRIMARY KEY,
    username   VARCHAR(40)  NOT NULL,
    email      VARCHAR(80)  NOT NULL UNIQUE,
    password   VARCHAR(100) NOT NULL,
    address    VARCHAR(100),
    phone      VARCHAR(12),
    avatar_url VARCHAR(100)
);

CREATE TABLE IF NOT EXISTS Pizzas
(
    id SERIAL PRIMARY KEY,
    title VARCHAR(50) NOT NULL UNIQUE,
    ingredients VARCHAR(20)[] NOT NULL,
    price SMALLINT NOT NULL,
    image_url VARCHAR(100) NOT NULL
);