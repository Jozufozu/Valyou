
CREATE TABLE entries (
    id              BIGINT      PRIMARY KEY default id_generator(),
    author          BIGINT      NOT NULL    REFERENCES profiles ON UPDATE CASCADE ON DELETE CASCADE,
    journal         BIGINT      NOT NULL    REFERENCES journals ON UPDATE CASCADE ON DELETE CASCADE,
    created         timestamp   NOT NULL    default now(),
    modified        timestamp,
    modifiedc       timestamp,
    content         VARCHAR     NOT NULL,
    significance    FLOAT,
    hidden          BOOLEAN     NOT NULL    default FALSE
);

CREATE TABLE entry_tags (
    entry   BIGINT  REFERENCES entries ON UPDATE CASCADE ON DELETE CASCADE,
    tag     VARCHAR NOT NULL,
    CONSTRAINT entry_tags_pkey PRIMARY KEY (entry, tag)
);