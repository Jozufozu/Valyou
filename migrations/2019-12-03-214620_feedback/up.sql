CREATE TABLE feedback (
    id          BIGINT      PRIMARY KEY default id_generator(),
    author      BIGINT      NOT NULL    REFERENCES profiles,
    entry       BIGINT      NOT NULL    REFERENCES entries,
    created     timestamp   NOT NULL    default now(),
    modified    timestamp,
    content     VARCHAR     NOT NULL,
    starred     BOOLEAN     NOT NULL    default FALSE
);