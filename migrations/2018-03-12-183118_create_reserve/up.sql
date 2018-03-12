CREATE TABLE reserve_type (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

INSERT INTO reserve_type (id, name) VALUES
       (5, 'Depleted'),
       (4, 'Low'),
       (3, 'Common'),
       (2, 'Major'),
       (1, 'Pristine')
