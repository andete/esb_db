ALTER TABLE faction
DROP COLUMN state_id;
CREATE TABLE faction_state (
       id SERIAL PRIMARY KEY,
       stamp TIMESTAMPTZ NOT NULL,
       faction_id INTEGER NOT NULL REFERENCES faction (id),
       state_id INTEGER NOT NULL REFERENCES state (id)
       );

CREATE UNIQUE INDEX faction_state_stamp_idx ON faction_state (stamp);
CREATE UNIQUE INDEX faction_state_faction_idx ON faction_state (faction_id);
