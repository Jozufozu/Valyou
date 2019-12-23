CREATE TYPE visibility AS ENUM ('public', 'private', 'friends');

CREATE TABLE profiles (
    id          BIGINT  PRIMARY KEY REFERENCES accounts ON UPDATE CASCADE ON DELETE CASCADE,
    visibility  visibility  NOT NULL    default 'private',
    summary     VARCHAR,
    bio         VARCHAR,
    modified    timestamp
);

CREATE TABLE usernames (
    id          BIGINT      PRIMARY KEY REFERENCES profiles ON UPDATE CASCADE ON DELETE CASCADE,
    handle      VARCHAR     NOT NULL,
    numbers     SMALLINT    NOT NULL    CHECK ( numbers < 10000 and numbers > 0 ),
    modified    timestamp,

    UNIQUE (handle, numbers)
);