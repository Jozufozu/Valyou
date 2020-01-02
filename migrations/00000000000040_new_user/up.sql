create or replace function insert_new_account() returns trigger as
$$
declare
    uid bigint;
begin
    insert into accounts (email, hash) values (new.email, new.hash) returning (userid) into uid;
    insert into account_age (userid) values (uid);
    insert into profiles (userid) values (uid);
    insert into usernames (userid, username) values (uid, new.username);

    return new;
end;
$$ language plpgsql;

create view new_account (email, hash, username) as
values (text '', text '', text '');

create trigger new_account_trigger
    instead of insert
    on new_account
    for each row
execute procedure insert_new_account();