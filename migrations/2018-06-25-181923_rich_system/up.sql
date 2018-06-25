CREATE VIEW rich_system AS
SELECT
        system.id,
        system.name,
        s.id as security_id,
        s.name as security,
        system.needs_permit,
        system.x,
        system.y,
        system.z,
        system.simbad_ref,
        r.id as reserve_type_id,
        r.name as reserve_type,
        system.is_populated,
        system.edsm_id,
        system.updated_at
FROM system
     INNER JOIN security AS s ON system.security_id = s.id
     INNER JOIN reserve_type AS r ON system.reserve_type_id = r.id;
