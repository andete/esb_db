table! {
    allegiance (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    controlling (id) {
        id -> Int4,
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
        state_id -> Nullable<Int4>,
        government_id -> Nullable<Int4>,
        home_system_id -> Nullable<Int4>,
        is_player_faction -> Bool,
        updated_at -> Timestamptz,
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
        id -> Int4,
        stamp -> Timestamptz,
        system_id -> Int4,
        allegiance_id -> Int4,
        power_state_id -> Int4,
    }
}

joinable!(controlling -> faction (faction_id));
joinable!(controlling -> system (system_id));
joinable!(faction -> allegiance (allegiance_id));
joinable!(faction -> government (government_id));
joinable!(faction -> state (state_id));
joinable!(faction -> system (home_system_id));
joinable!(power -> allegiance (allegiance_id));
joinable!(system -> reserve_type (reserve_type_id));
joinable!(system -> security (security_id));
joinable!(system_power -> allegiance (allegiance_id));
joinable!(system_power -> power_state (power_state_id));
joinable!(system_power -> system (system_id));

allow_tables_to_appear_in_same_query!(
    allegiance,
    controlling,
    economy,
    faction,
    government,
    power,
    power_state,
    reserve_type,
    security,
    state,
    system,
    system_power,
);
