CREATE TABLE security (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

INSERT INTO security (id, name) VALUES
       (16, 'Low'),
       (32, 'Medium'),
       (48, 'High'),
       (64, 'Anarchy'),
       (80, 'Lawless')
