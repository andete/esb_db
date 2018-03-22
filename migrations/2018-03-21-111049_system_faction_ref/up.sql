ALTER TABLE system
ADD FOREIGN KEY(controlling_minor_faction_id)
REFERENCES faction (id);
