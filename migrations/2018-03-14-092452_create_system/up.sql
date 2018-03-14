CREATE TABLE system (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL UNIQUE,
  allegiance_id SERIAL REFERENCES allegiance (id),
  state_id SERIAL REFERENCES state (id),
  government_id SERIAL REFERENCES government (id),
  security_id SERIAL REFERENCES security (id),
  needs_permit BOOLEAN DEFAULT FALSE,
  power_state_id SERIAL REFERENCES power_state (id),
  x DOUBLE PRECISION NOT NULL,
  y DOUBLE PRECISION NOT NULL,
  z DOUBLE PRECISION NOT NULL,
  simbad_ref VARCHAR DEFAULT '',
  controlling_minor_faction_id SERIAL, -- TODO REF
  reserve_type_id SERIAL REFERENCES reserve_type (id),
  is_populated BOOLEAN DEFAULT FALSE,
  edsm_id SERIAL,
  updated_at TIMESTAMP
);

CREATE UNIQUE INDEX system_name_idx ON system (name);
CREATE UNIQUE INDEX system_edsm_id_idx ON system (edsm_id);
