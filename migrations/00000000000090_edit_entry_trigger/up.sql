CREATE FUNCTION edit_entry_guard() RETURNS trigger AS $$
BEGIN
    select now() into new.modified;
    if (new.content notnull) then
        if ((floor(extract(epoch from (now()-old.created)))) > 86400) then
            raise exception 'edit_after_day';
        end if;

        select now() into new.modifiedc;
    end if;

    return new;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER edit_entry_guard
    BEFORE UPDATE
    ON entries
    FOR EACH ROW
EXECUTE PROCEDURE edit_entry_guard();