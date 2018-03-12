;CREATE TABLE state (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

INSERT INTO state (id, name) VALUES
      (16, 'Boom'),
      (32, 'Bust'),
      (37, 'Famine'),
      (48, 'Civil Unrest'),
      (64, 'Civil War'),
      (65, 'Election'),
      (67, 'Expansion'),
      (69, 'Lockdown'),
      (72, 'Outbreak'),
      (73, 'War'),
      (80, 'None'),
      (96, 'Retreat'),
      (101, 'Investment')
