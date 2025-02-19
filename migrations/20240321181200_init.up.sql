CREATE TABLE IF NOT EXISTS Users
(
    id       UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50)  NOT NULL,
    email    VARCHAR(80)  NOT NULL UNIQUE,
    password VARCHAR(100) NOT NULL,
    address  VARCHAR(100),
    phone    VARCHAR(12)
);

CREATE TABLE IF NOT EXISTS Pizza
(
    id          SERIAL PRIMARY KEY,
    title       VARCHAR(50)   NOT NULL UNIQUE,
    ingredients VARCHAR(20)[] NOT NULL,
    price       SMALLINT      NOT NULL
);
