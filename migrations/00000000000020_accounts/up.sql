create table accounts (
    userid      bigint      primary key default id_generator(),
    email       varchar     not null    unique,
    hash        varchar     not null,
    created     timestamp   not null    default now(),
    modified    timestamp,

    constraint proper_email check (email ~* '^[a-za-z0-9._%-]+@[a-za-z0-9.-]+[.][a-za-z]+$')
);