CREATE FUNCTION check_owner(journalid BIGINT, requiredowner BIGINT) RETURNS BOOLEAN AS $$
BEGIN
    return exists(select 1 from journals where (journals.id=journalid and journals.owner=requiredowner));
END;
$$ LANGUAGE plpgsql;

ALTER TABLE entries
ADD CONSTRAINT author_owns_journal CHECK (check_owner(journal, author));