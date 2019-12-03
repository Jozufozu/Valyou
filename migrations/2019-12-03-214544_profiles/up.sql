
CREATE TABLE profiles (
    id      BIGINT  PRIMARY KEY,
    summary VARCHAR,
    bio     VARCHAR
);

CREATE TYPE visibility AS ENUM ('public', 'private', 'friends');

CREATE TABLE user_visibility (
    id          BIGINT      PRIMARY KEY,
    visibility  visibility  NOT NULL
);

CREATE TABLE usernames (
    id      BIGINT  PRIMARY KEY,
    handle  VARCHAR NOT NULL    UNIQUE
);