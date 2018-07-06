table! {
    allegiance (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    controlling (id) {
        id -> Int8,
        stamp -> Timestamptz,
        system_id -> Int4,
        faction_id -> Nullable<Int4>,
    }
}

table! {
    economy (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    faction (id) {
        id -> Int4,
        name -> Varchar,
        allegiance_id -> Nullable<Int4>,
        government_id -> Nullable<Int4>,
        home_system_id -> Nullable<Int4>,
        is_player_faction -> Bool,
        updated_at -> Timestamptz,
    }
}

table! {
    faction_state (id) {
        id -> Int8,
        stamp -> Timestamptz,
        faction_id -> Int4,
        state_id -> Int4,
    }
}

table! {
    government (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    power (id) {
        id -> Int4,
        name -> Varchar,
        allegiance_id -> Nullable<Int4>,
    }
}

table! {
    power_state (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    presence (id) {
        id -> Int8,
        stamp -> Timestamptz,
        system_id -> Int4,
        faction_id -> Int4,
        state_id -> Nullable<Int4>,
        influence -> Float4,
    }
}

table! {
    reserve_type (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    security (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    state (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    system (id) {
        id -> Int4,
        name -> Varchar,
        security_id -> Nullable<Int4>,
        needs_permit -> Nullable<Bool>,
        x -> Float8,
        y -> Float8,
        z -> Float8,
        simbad_ref -> Nullable<Varchar>,
        reserve_type_id -> Nullable<Int4>,
        is_populated -> Nullable<Bool>,
        edsm_id -> Nullable<Int4>,
        updated_at -> Nullable<Timestamptz>,
    }
}

table! {
    system_power (id) {
        id -> Int8,
        stamp -> Timestamptz,
        system_id -> Int4,
        power_state_id -> Nullable<Int4>,
    }
}

table! {
    rich_faction (id) {
        id -> Int4,
        name -> Varchar,
        allegiance_id -> Nullable<Int4>,
        allegiance -> Nullable<Varchar>,
        government_id -> Nullable<Int4>,
        government -> Nullable<Varchar>,
        home_system_id -> Nullable<Int4>,
        home_system -> Nullable<Varchar>,
        is_player_faction -> Bool,
        updated_at -> Timestamptz,
    }
}

table! {
    rich_system (id) {
        id -> Int4,
        name -> Varchar,
        security_id -> Nullable<Int4>,
        security -> Nullable<Varchar>,
        needs_permit -> Nullable<Bool>,
        x -> Float8,
        y -> Float8,
        z -> Float8,
        simbad_ref -> Nullable<Varchar>,
        reserve_type_id -> Nullable<Int4>,
        reserve_type -> Nullable<Varchar>,
        is_populated -> Nullable<Bool>,
        edsm_id -> Nullable<Int4>,
        updated_at -> Nullable<Timestamptz>,
    }
}

joinable!(controlling -> faction (faction_id));
joinable!(controlling -> rich_faction (faction_id));
joinable!(controlling -> system (system_id));
joinable!(controlling -> rich_system (system_id));

joinable!(faction -> allegiance (allegiance_id));
joinable!(faction -> government (government_id));
joinable!(faction -> system (home_system_id));
joinable!(faction -> rich_system (home_system_id));

joinable!(faction_state -> faction (faction_id));
joinable!(faction_state -> rich_faction (faction_id));
joinable!(faction_state -> state (state_id));

joinable!(power -> allegiance (allegiance_id));

joinable!(presence -> faction (faction_id));
joinable!(presence -> rich_faction (system_id));
joinable!(presence -> state (state_id));
joinable!(presence -> system (system_id));
joinable!(presence -> rich_system (system_id));

joinable!(system -> reserve_type (reserve_type_id));
joinable!(system -> security (security_id));

joinable!(system_power -> power_state (power_state_id));
joinable!(system_power -> system (system_id));
joinable!(system_power -> rich_system (system_id));

allow_tables_to_appear_in_same_query!(
    allegiance,
    controlling,
    economy,
    faction,
    faction_state,
    government,
    power,
    power_state,
    presence,
    reserve_type,
    rich_faction,
    rich_system,
    security,
    state,
    system,
    system_power,
);
