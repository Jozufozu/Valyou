CREATE TYPE status AS ENUM ('pending_first_second', 'pending_second_first', 'friends', 'block_first_second', 'block_second_first', 'block_both');
CREATE TABLE relations (
    user_from   BIGINT      NOT NULL    REFERENCES profiles ON UPDATE CASCADE ON DELETE CASCADE,
    user_to     BIGINT      NOT NULL    REFERENCES profiles ON UPDATE CASCADE ON DELETE CASCADE,
    since       timestamp   NOT NULL    default now(),
    status      status      NOT NULL,

    CONSTRAINT  id_order        CHECK ( user_from < user_to ),
    CONSTRAINT  relations_pk    PRIMARY KEY (user_from, user_to)
);