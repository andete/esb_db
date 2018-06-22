CREATE TABLE presence (
       id SERIAL PRIMARY KEY,
       stamp TIMESTAMPTZ NOT NULL,
       system_id INTEGER NOT NULL REFERENCES system (id),
       faction_id INTEGER NOT NULL REFERENCES faction (id),
       state_id INTEGER REFERENCES state (id),
       influence REAL
       );
       

CREATE INDEX presence_system_idx ON presence (system_id);
CREATE INDEX presence_stamp_idx ON presence (stamp);
CREATE INDEX presence_faction_idx ON presence (faction_id);
