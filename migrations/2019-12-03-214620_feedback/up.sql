CREATE TABLE feedback (
    id          BIGINT      PRIMARY KEY,
    author      BIGINT      NOT NULL    REFERENCES profiles,
    entry       BIGINT      NOT NULL    REFERENCES entries,
    created     timestamp   NOT NULL    default now(),
    modified    timestamp,
    content     VARCHAR     NOT NULL,
    starred     BOOLEAN     NOT NULL    default FALSE
);

CREATE TABLE feedback_replies (
    child       BIGINT  PRIMARY KEY     REFERENCES feedback,
    parent      BIGINT  NOT NULL        REFERENCES feedback
);