CREATE TABLE system (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL UNIQUE,
  allegiance_id INTEGER REFERENCES allegiance (id),
  state_id INTEGER REFERENCES state (id),
  government_id INTEGER REFERENCES government (id),
  security_id INTEGER REFERENCES security (id),
  needs_permit BOOLEAN DEFAULT FALSE,
  power_state_id INTEGER REFERENCES power_state (id),
  x DOUBLE PRECISION NOT NULL,
  y DOUBLE PRECISION NOT NULL,
  z DOUBLE PRECISION NOT NULL,
  simbad_ref VARCHAR DEFAULT '',
  controlling_minor_faction_id INTEGER, -- TODO REF
  reserve_type_id INTEGER REFERENCES reserve_type (id),
  is_populated BOOLEAN DEFAULT FALSE,
  edsm_id INTEGER,
  updated_at TIMESTAMPTZ
);

CREATE UNIQUE INDEX system_name_idx ON system (name);
CREATE UNIQUE INDEX system_edsm_id_idx ON system (edsm_id);
