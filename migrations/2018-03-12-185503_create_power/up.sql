CREATE TABLE power (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL UNIQUE,
  allegiance_id INTEGER REFERENCES allegiance (id)
);

CREATE UNIQUE INDEX power_name_idx ON power (name);

INSERT INTO power (id, name, allegiance_id) VALUES
       ( 1, 'Edmund Mahon',         1),
       (16, 'Aisling Duval',        2),
       (17, 'Arissa Lavigny-Duval', 2),
       (18, 'Zemina Torval',        2),
       (19, 'Denton Patreus',       2),
       (32, 'Felicia Winters',      3),
       (33, 'Zachary Hudson',       3),
       (48, 'Archon Delaine',       4),
       (49, 'Li Yong-Rui',          4),
       (50, 'Pranav Antal',         4),
       (51, 'Yuri Grom',            4)
