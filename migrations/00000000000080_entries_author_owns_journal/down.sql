ALTER TABLE entries
DROP CONSTRAINT author_owns_journal;

DROP FUNCTION check_owner;