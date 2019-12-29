create or replace function check_owner(jid bigint, requiredowner bigint) returns boolean as
$$
begin
    return exists(select 1 from journals j where (j.journalid = jid and j.owner = requiredowner));
end;
$$ language plpgsql;

create table entries
(
    entryid      bigint primary key default id_generator(),
    author       bigint    not null references profiles on update cascade on delete cascade,
    journal      bigint    not null references journals on update cascade on delete cascade,
    created      timestamp not null default now(),
    modified     timestamp,
    modifiedc    timestamp,
    content      varchar   not null,
    significance float,
    hidden       boolean   not null default false,
    constraint author_owns_journal check (check_owner(journal, author))
);

create table entry_tags
(
    entry bigint references entries on update cascade on delete cascade,
    tag   varchar(48) not null check ( tag ~* '[^\s,]{3,48}' ),
    primary key (entry, tag)
);

create view full_entries as
    select entryid, author, journal, created, modified, modifiedc, content, significance, hidden,
           (select string_agg(tag,',')
            from entry_tags
            where entry=entryid) as tags
    from entries;

create view visible_entries as
    select entryid, author, journal, created, modified, modifiedc, content, significance, tags
    from full_entries
    where not hidden;

create view hidden_entries as
    select entryid, author, journal, created, modified, modifiedc, content, significance, tags
    from full_entries
    where hidden;

create or replace function edit_entry() returns trigger as
$$
begin
    select now() into new.modified;
    if (new.content != old.content) then
        if ((floor(extract(epoch from (now() - old.created)))) > 86400) then
            raise check_violation using constraint = 'edit_after_day';
        end if;

        select now() into new.modifiedc;
    end if;

    return new;
end;
$$ language plpgsql;

create trigger edit_entry
    before update
    on entries
    for each row
execute procedure edit_entry();

create trigger timestamp_guard
    before update of created, modified, modifiedc
    on entries
    for each row
execute procedure timestamp_guard();