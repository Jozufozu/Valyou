
create table journals (
    journalid   bigint      primary key default id_generator(),
    owner       bigint      not null    references profiles on update cascade on delete cascade,
    title       varchar(32) not null,
    created     timestamp   not null    default now(),
    modified    timestamp,
    description varchar(240),
    visibility  visibility  not null,
    color       int         not null    default 0
);