
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

create or replace function edit_journal() returns trigger as $$
begin
    select now() into new.modified;
    return new;
end;
$$ language plpgsql;

create trigger edit_journal
    before update
    on journals
    for each row
execute procedure edit_journal();

create trigger timestamp_guard
    before update of created, modified
    on journals
    for each row
execute procedure timestamp_guard();