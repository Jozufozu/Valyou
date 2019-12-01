CREATE TABLE accounts (
  id        BIGINT PRIMARY KEY  NOT NULL,
  username  VARCHAR             NOT NULL,
  passhash  VARCHAR             NOT NULL,
  email     VARCHAR             NOT NULL,
  joined    timestamp           NOT NULL,
  modified  timestamp,
  name      VARCHAR,
  phone     VARCHAR,
  bio       VARCHAR
)