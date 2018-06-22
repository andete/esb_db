ALTER TABLE system ADD COLUMN allegiance_id INTEGER REFERENCES allegiance (id);
ALTER TABLE system ADD COLUMN state_id INTEGER REFERENCES state (id);
ALTER TABLE system ADD COLUMN government_id INTEGER REFERENCES government (id);
ALTER TABLE system ADD COLUMN power_state_id INTEGER REFERENCES power_state (id);
