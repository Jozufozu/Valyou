CREATE TABLE feedback (
    id          BIGINT      PRIMARY KEY,
    author      BIGINT      NOT NULL    REFERENCES profiles,
    entry       BIGINT      NOT NULL    REFERENCES entries,
    created     timestamp   NOT NULL    default now(),
    modified    timestamp,
    content     VARCHAR     NOT NULL,
    reply_to    BIGINT      REFERENCES feedback
);