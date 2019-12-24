create or replace function gen_discriminator(handle varchar) returns smallint as $$
declare
    num smallint;
begin
    if (select count(1) from usernames where usernames.username=handle)=9999 then
        raise exception 'handle_not_available';
    end if;

    select tag from generate_series(1, 9999) as tag
    left join usernames on usernames.username=handle and usernames.discriminator=tag
    where usernames.username isnull
    order by random()
    limit 1
    into num;

    return num;
end;
$$ language plpgsql;

create or replace function insert_new_account() returns trigger as $$
declare
    dis smallint;
    uid bigint;
begin
    select gen_discriminator(new.username) into dis;

    insert into accounts (email, hash) values (new.email, new.hash) returning (userid) into uid;
    insert into profiles (userid) values (uid);
    insert into usernames (userid, username, discriminator) values (uid, new.username, dis);

    return new;
end;
$$ language plpgsql;

create view new_account (email, hash, username) as values (text '', text '', text '');

create trigger new_account_trigger
instead of insert
on new_account
for each row
execute procedure insert_new_account();