CREATE TABLE system_power (
       id serial PRIMARY KEY,
       stamp TIMESTAMPTZ NOT NULL,
       system_id INTEGER NOT NULL REFERENCES system (id),
       allegiance_id INTEGER NOT NULL REFERENCES allegiance (id),
       power_state_id INTEGER NOT NULL REFERENCES power_state (id)
       );

CREATE INDEX system_power_stamp_idx ON system_power (stamp);
CREATE INDEX system_power_system_idx ON system_power (system_id);
