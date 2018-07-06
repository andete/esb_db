ALTER TABLE presence ALTER COLUMN state_id DROP NOT NULL;
UPDATE presence set state_id = NULL WHERE state_id = 0;
DELETE FROM state WHERE id = 0;
