create type visibility as enum ('public', 'private', 'friends');

create table profiles
(
    userid     bigint primary key references accounts on update cascade on delete cascade,
    visibility visibility not null default 'private',
    summary    varchar(120),
    bio        varchar(400),
    modified   timestamp
);

create table usernames
(
    userid        bigint primary key references profiles on update cascade on delete cascade,
    username      varchar(32)  not null check ( username ~* '\S{3,32}' ),
    discriminator smallint not null check ( discriminator < 10000 and discriminator > 0 ),
    modified      timestamp,

    unique (username, discriminator)
);

create view searchable as
select u.userid, u.username, u.discriminator, p.summary, p.bio
from profiles p
         inner join usernames u on p.userid = u.userid
where p.visibility != 'private';

create or replace function gen_discriminator(handle varchar) returns smallint as
$$
declare
    num smallint;
begin
    if (select count(1) from usernames where usernames.username = handle) = 9999 then
        raise exception 'handle_not_available';
    end if;

    select tag
    from generate_series(1, 9999) as tag
             left join usernames on usernames.username = handle and usernames.discriminator = tag
    where usernames.username isnull
    order by random()
    limit 1
    into num;

    return num;
end;
$$ language plpgsql;

create or replace function edit_profile() returns trigger as
$$
begin
    select now() into new.modified;
    return new;
end;
$$ language plpgsql;

create trigger edit_profile
    before update
    on profiles
    for each row
execute procedure edit_profile();

create or replace function edit_username() returns trigger as
$$
begin
    select now() into new.modified;
    return new;
end;
$$ language plpgsql;

create trigger edit_username
    before update
    on usernames
    for each row
execute procedure edit_username();

create or replace function set_discriminator() returns trigger as
$$
begin
    select gen_discriminator(new.username) into new.discriminator;
    return new;
end;
$$ language plpgsql;

create trigger update_discriminator
    before update of username
    on usernames
    for each row
execute procedure set_discriminator();

create trigger set_discriminator
    before insert
    on usernames
    for each row
execute procedure set_discriminator();

create trigger timestamp_guard
    before update of modified
    on profiles
    for each row
execute procedure timestamp_guard();

create trigger timestamp_guard
    before update of modified
    on usernames
    for each row
execute procedure timestamp_guard();
