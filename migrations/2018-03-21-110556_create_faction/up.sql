CREATE TABLE faction (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL UNIQUE,
  allegiance_id INTEGER REFERENCES allegiance (id),
  state_id INTEGER REFERENCES state (id),
  government_id INTEGER REFERENCES government (id),
  home_system_id INTEGER REFERENCES system (id),
  is_player_faction BOOLEAN NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL
);

CREATE UNIQUE INDEX faction_name_idx ON faction (name);
