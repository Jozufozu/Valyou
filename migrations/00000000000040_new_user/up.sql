CREATE FUNCTION gen_discriminator(handle VARCHAR) RETURNS SMALLINT AS $$
DECLARE
    num smallint;
BEGIN
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
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION insert_new_account() RETURNS trigger AS $$
DECLARE
    num smallint;
    userid bigint;
BEGIN
    select gen_discriminator(new.username) into num;

    insert into accounts (email, hash) values (new.email, new.hash) returning (id) into userid;
    insert into profiles (id) values (userid);
    insert into usernames (id, username, discriminator) values (userid, new.username, num);

    return new;
END;
$$ LANGUAGE plpgsql;

create view new_account (email, hash, username) as values (text '', text '', text '');

CREATE TRIGGER new_account_trigger
    INSTEAD OF INSERT
    ON new_account
    FOR EACH ROW
EXECUTE PROCEDURE insert_new_account();