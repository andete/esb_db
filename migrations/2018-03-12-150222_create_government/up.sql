CREATE TABLE government (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

INSERT INTO government (id, name) VALUES
       (16, 'Anarchy'),
       (32, 'Communism'),
       (48, 'Confederacy'),
       (64, 'Corporate'),
       (80, 'Cooperative'),
       (96, 'Democracy'),
       (112, 'Dictatorship'),
       (128, 'Feudal'),
       (144, 'Patronage'),
       (150, 'Prison Colony'),
       (160, 'Theocracy'),
       (208, 'Prison')
