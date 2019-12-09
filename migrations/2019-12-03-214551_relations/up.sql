CREATE TYPE status AS ENUM ('pending_first_second', 'pending_second_first', 'friends', 'block_first_second', 'block_second_first', 'block_both');
CREATE TABLE relations (
    id          BIGINT      PRIMARY KEY default id_generator(),
    user_from   BIGINT      NOT NULL    REFERENCES profiles,
    user_to     BIGINT      NOT NULL    REFERENCES profiles,
    since       timestamp   NOT NULL    default now(),
    status      status      NOT NULL
);