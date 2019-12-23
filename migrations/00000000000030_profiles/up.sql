CREATE TYPE visibility AS ENUM ('public', 'private', 'friends');

CREATE TABLE profiles (
    id          BIGINT  PRIMARY KEY REFERENCES accounts ON UPDATE CASCADE ON DELETE CASCADE,
    visibility  visibility  NOT NULL    default 'private',
    summary     VARCHAR,
    bio         VARCHAR,
    modified    timestamp
);

CREATE TABLE usernames (
    id              BIGINT      PRIMARY KEY REFERENCES profiles ON UPDATE CASCADE ON DELETE CASCADE,
    username        VARCHAR     NOT NULL,
    discriminator   SMALLINT    NOT NULL    CHECK ( discriminator < 10000 and discriminator > 0 ),
    modified        timestamp,

    UNIQUE (username, discriminator)
);