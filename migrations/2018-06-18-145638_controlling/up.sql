CREATE TABLE controlling (
       id serial PRIMARY KEY,
       stamp TIMESTAMPTZ NOT NULL,
       system_id INTEGER NOT NULL REFERENCES system (id),
       faction_id INTEGER REFERENCES faction (id)
       );

CREATE INDEX controlling_system_stamp_idx ON controlling (stamp);
CREATE INDEX controlling_system_id_idx ON controlling (system_id);
CREATE INDEX controlling_faction_id_idx ON controlling (faction_id);
