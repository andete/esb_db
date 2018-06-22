DROP TABLE faction_state;
ALTER TABLE faction
ADD COLUMN state_id INTEGER REFERENCES state (id);
