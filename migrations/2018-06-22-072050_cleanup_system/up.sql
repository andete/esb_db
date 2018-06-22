-- remove columns that really are just a function of the
-- controlling minor faction
ALTER TABLE system DROP COLUMN allegiance_id;
ALTER TABLE system DROP COLUMN state_id;
ALTER TABLE system DROP COLUMN government_id;
ALTER TABLE system DROP COLUMN power_state_id;
