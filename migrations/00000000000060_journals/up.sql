
CREATE TABLE journals (
    id          BIGINT      PRIMARY KEY default id_generator(),
    owner       BIGINT      NOT NULL    REFERENCES profiles ON UPDATE CASCADE ON DELETE CASCADE,
    name        VARCHAR     NOT NULL,
    created     timestamp   NOT NULL    default now(),
    modified    timestamp,
    description VARCHAR,
    visibility  visibility  NOT NULL
);