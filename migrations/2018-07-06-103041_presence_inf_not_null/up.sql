UPDATE presence SET influence = 0.0 WHERE influence IS NULL;
ALTER TABLE presence ALTER COLUMN influence SET NOT NULL;
