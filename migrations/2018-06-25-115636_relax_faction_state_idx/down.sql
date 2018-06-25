DROP INDEX faction_state_stamp_idx;
DROP INDEX faction_state_faction_idx;
CREATE UNIQUE INDEX faction_state_stamp_idx ON faction_state (stamp);
CREATE UNIQUE INDEX faction_state_faction_idx ON faction_state (faction_id);
