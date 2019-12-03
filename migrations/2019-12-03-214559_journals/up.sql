
CREATE TABLE journals (
    id          BIGINT      PRIMARY KEY,
    owner       BIGINT      NOT NULL    REFERENCES profiles,
    name        VARCHAR     NOT NULL,
    created     timestamp   NOT NULL    default now(),
    modified    timestamp,
    description VARCHAR,
    visibility  visibility
);