create or replace function can_see_entry(me bigint, author bigint, journal bigint) returns boolean as
$$
declare
    jvis visibility;
    pvis visibility;
    rstatus status;
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

    if me < author then
        select status from relations where user_from=me and user_to=author into rstatus;

        if rstatus='block_second_first' then
            return false;
        end if;
    else
        select status from relations where user_from=author and user_to=me into rstatus;

        if rstatus='block_first_second' then
            return false;
        end if;
    end if;

    if rstatus='block_both' then
        return false;
    end if;

    return (jvis='public' and pvis='public') or rstatus='friends';
end;
$$ language plpgsql;