CREATE TABLE power_state (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL UNIQUE
);

CREATE UNIQUE INDEX power_state_name_idx ON power_state (name);

INSERT INTO power_state (id, name) VALUES
       (16, 'Control'),
       (32, 'Exploited'),
       (48, 'Contested')
