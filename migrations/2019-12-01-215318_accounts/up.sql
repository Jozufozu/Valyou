CREATE TABLE accounts (
    id          BIGINT      PRIMARY KEY,
    email       VARCHAR     NOT NULL    UNIQUE,
    passhash    VARCHAR     NOT NULL,
    created     timestamp   NOT NULL    default now(),
    modified    timestamp,
    phone       VARCHAR
);