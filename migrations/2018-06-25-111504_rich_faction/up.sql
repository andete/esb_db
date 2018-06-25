CREATE VIEW rich_faction AS
SELECT
        faction.id,
        faction.name,
        a.id AS allegiance_id,
        a.name AS allegiance,
        g.id AS government_id,
        g.name AS government,
        h.id AS home_system_id,
        h.name AS home_system,
        faction.is_player_faction,
        faction.updated_at
FROM faction
     INNER JOIN allegiance AS A ON faction.allegiance_id = a.id
     INNER JOIN government AS g ON faction.government_id = g.id
     INNER JOIN system AS h ON faction.home_system_id = h.id;
