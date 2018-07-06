INSERT INTO state (id, name) VALUES
      (0, 'Gone');

UPDATE presence SET state_id = 0 WHERE state_id IS NULL;
ALTER TABLE presence ALTER COLUMN state_id SET NOT NULL;
