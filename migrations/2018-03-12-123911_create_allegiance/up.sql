CREATE TABLE allegiance (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL UNIQUE
);

CREATE UNIQUE INDEX allegiance_name_idx ON allegiance (name);

INSERT INTO allegiance (id, name) VALUES
       (1, 'Alliance'),
       (2, 'Empire'),
       (3, 'Federation'),
       (4, 'Independant'),
       (7, 'Pilots Federation'),
