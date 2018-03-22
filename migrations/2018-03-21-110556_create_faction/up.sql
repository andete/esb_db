CREATE TABLE faction (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL UNIQUE,
  allegiance_id SERIAL REFERENCES allegiance (id),
  state_id SERIAL REFERENCES state (id),
  government_id SERIAL REFERENCES government (id),
  home_system_id SERIAL REFERENCES system (id),
  is_player_faction BOOLEAN,
  updated_at TIMESTAMPTZ
);

CREATE UNIQUE INDEX faction_name_idx ON faction (name);
