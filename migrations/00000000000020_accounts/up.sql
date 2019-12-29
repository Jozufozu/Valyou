create table accounts (
    userid          bigint      primary key default id_generator(),
    email           varchar     not null    unique,
    hash            varchar     not null,
    created         timestamp   not null    default now(),
    modified        timestamp,
    modified_hash   timestamp,

    constraint proper_email check (email ~* '^[a-za-z0-9._%-]+@[a-za-z0-9.-]+[.][a-za-z]+$')
);

create or replace function edit_account() returns trigger as $$
begin
    select now() into new.modified;
    return new;
end;
$$ language plpgsql;

create trigger edit_account
    before update
    on accounts
    for each row
execute procedure edit_account();

create or replace function edit_account_hash() returns trigger as $$
begin
    select now() into new.modified_hash;
    return new;
end;
$$ language plpgsql;

create trigger edit_account_hash
    before update of hash
    on accounts
    for each row
execute procedure edit_account_hash();

create trigger timestamp_guard
    before update of created, modified, modified_hash
    on accounts
    for each row
execute procedure timestamp_guard();