CREATE TABLE economy (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL UNIQUE
);

CREATE UNIQUE INDEX economy_name_idx ON economy (name);

INSERT INTO economy (id, name) VALUES
       (1, 'Agriculture'),
       (2, 'Extraction'),
       (3, 'High Tech'),
       (4, 'Industrial'),
       (5, 'Military'),
       (6, 'Refinery'),
       (7, 'Service'),
       (8, 'Terraforming'),
       (9, 'Tourism'),
       (10, 'None'),
       (11, 'Colony')
