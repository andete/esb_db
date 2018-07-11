CREATE TABLE state_pending (
       id SERIAL PRIMARY KEY,
       faction_state_id INTEGER NOT NULL REFERENCES faction_state (id),
       state_id INTEGER NOT NULL REFERENCES state (id),
       trend INTEGER NOT NULL
       );

CREATE TABLE state_recovery (
       id SERIAL PRIMARY KEY,
       faction_state_id INTEGER NOT NULL REFERENCES faction_state (id),
       state_id INTEGER NOT NULL REFERENCES state (id)
       );

CREATE INDEX state_pending_faction_state_id_idx ON state_pending (faction_state_id);
CREATE INDEX state_recovery_faction_state_id_idx ON state_recovery (faction_state_id);
