table! {
    allegiance (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    economy (id) {
        id -> Int4,
        name -> Varchar,
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
        allegiance_id -> Int4,
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
        allegiance_id -> Int4,
        state_id -> Int4,
        government_id -> Int4,
        security_id -> Int4,
        needs_permit -> Nullable<Bool>,
        power_state_id -> Int4,
        x -> Float8,
        y -> Float8,
        z -> Float8,
        simbad_ref -> Nullable<Varchar>,
        controlling_minor_faction_id -> Int4,
        reserve_type_id -> Int4,
        is_populated -> Nullable<Bool>,
        edsm_id -> Int4,
        updated_at -> Nullable<Timestamptz>,
    }
}

joinable!(power -> allegiance (allegiance_id));
joinable!(system -> allegiance (allegiance_id));
joinable!(system -> government (government_id));
joinable!(system -> power_state (power_state_id));
joinable!(system -> reserve_type (reserve_type_id));
joinable!(system -> security (security_id));
joinable!(system -> state (state_id));

allow_tables_to_appear_in_same_query!(
    allegiance,
    economy,
    government,
    power,
    power_state,
    reserve_type,
    security,
    state,
    system,
);
