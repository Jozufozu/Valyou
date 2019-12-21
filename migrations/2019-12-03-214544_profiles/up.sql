CREATE TYPE visibility AS ENUM ('public', 'private', 'friends');

CREATE TABLE profiles (
    id          BIGINT  PRIMARY KEY REFERENCES accounts,
    visibility  visibility  NOT NULL,
    summary     VARCHAR,
    bio         VARCHAR,
    modified    timestamp
);

CREATE TABLE usernames (
    id      BIGINT  PRIMARY KEY REFERENCES profiles,
    handle  VARCHAR NOT NULL    UNIQUE,
    modified    timestamp
);