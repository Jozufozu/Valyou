create or replace function can_see(me bigint, author bigint, journal bigint) returns boolean as
$$
declare
    jvis visibility;
    pvis visibility;
    blocked boolean;
    friends boolean;
begin
    if me = author then
        return true;
    end if;

    select visibility from profiles where userid=author into pvis;

    if pvis='private' then
        return false;
    end if;

    select visibility from journals where journalid=journal into jvis;

    if jvis='private' then
        return false;
    end if;

    select is_blocked(me, author, blocked, friends);

    return not blocked and ((jvis='public' and pvis='public') or friends);
end;
$$ language plpgsql;

create or replace function can_see_user(me bigint, other bigint) returns boolean as
$$
declare
    pvis visibility;
    blocked boolean;
    friends boolean;
begin
    if me = other then
        return true;
    end if;

    select visibility from profiles where userid=other into pvis;

    if pvis='private' then
        return false;
    end if;

    select is_blocked(me, other, blocked, friends);

    return not blocked and (pvis='public' or friends);
end;
$$ language plpgsql;

create or replace function is_blocked(me bigint, other bigint, out blocked boolean, out friends boolean) as
$$
declare
    rstatus status;
begin
    if me < other then
        select status from relations where user_from=me and user_to=other into rstatus;

        if rstatus='block_second_first' then
            select true into blocked;
        end if;
    else
        select status from relations where user_from=other and user_to=me into rstatus;

        if rstatus='block_first_second' then
            select true into blocked;
        end if;
    end if;

    select blocked or rstatus='block_both' into blocked;
    select rstatus='friends' into friends;
end;
$$ language plpgsql;