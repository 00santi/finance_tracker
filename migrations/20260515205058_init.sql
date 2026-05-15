CREATE TABLE users
(
    id            SERIAL PRIMARY KEY,
    name          TEXT,
    email         TEXT      NOT NULL UNIQUE,
    password_hash TEXT      NOT NULL,
    created_at    TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE transactions
(
    id          SERIAL PRIMARY KEY,
    user_id     INTEGER REFERENCES users (id) ON DELETE CASCADE,
    amount      NUMERIC   NOT NULL,
    category    TEXT      NOT NULL,
    description TEXT,
    created_at  TIMESTAMP NOT NULL DEFAULT now()
);

