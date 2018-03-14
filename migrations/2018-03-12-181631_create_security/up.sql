CREATE TABLE security (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL UNIQUE
);

CREATE UNIQUE INDEX security_name_idx ON security (name);

INSERT INTO security (id, name) VALUES
       (16, 'Low'),
       (32, 'Medium'),
       (48, 'High'),
       (64, 'Anarchy'),
       (80, 'Lawless')
