CREATE TABLE allegiance (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

INSERT INTO allegiance (id, name) VALUES
       (1, 'Alliance'),
       (2, 'Empire'),
       (3, 'Federation'),
       (4, 'Independant')
