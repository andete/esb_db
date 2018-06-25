ALTER TABLE system_power
ADD COLUMN allegiance_id INTEGER REFERENCES allegiance (id);
