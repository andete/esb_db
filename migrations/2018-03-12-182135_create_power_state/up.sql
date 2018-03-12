CREATE TABLE power_state (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

INSERT INTO power_state (id, name) VALUES
       (16, 'Control'),
       (32, 'Exploited'),
       (48, 'Contested')
