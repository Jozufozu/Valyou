
CREATE TABLE entries (
    id              BIGINT      PRIMARY KEY default id_generator(),
    author          BIGINT      NOT NULL    REFERENCES profiles,
    journal         BIGINT      NOT NULL    REFERENCES journals,
    visibility      visibility  NOT NULL,
    created         timestamp   NOT NULL    default now(),
    modified        timestamp,
    modifiedc       timestamp,
    content         VARCHAR     NOT NULL,
    significance    FLOAT,
    hidden          BOOLEAN     NOT NULL    default FALSE
);

CREATE TABLE entry_tags (
    id      BIGINT  PRIMARY KEY default id_generator(),
    entry   BIGINT  REFERENCES entries,
    tag     VARCHAR NOT NULL
);