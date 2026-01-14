// @generated automatically by Diesel CLI.

diesel::table! {
    referral_codes (code) {
        owner_id -> Int8,
        code -> Text,
        is_active -> Bool,
        use_count -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    referral_owners (id) {
        id -> Int8,
        meta -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    referral_redemptions (id) {
        id -> Int8,
        code -> Text,
        meta -> Nullable<Jsonb>,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(referral_codes -> referral_owners (owner_id));
diesel::joinable!(referral_redemptions -> referral_codes (code));

diesel::allow_tables_to_appear_in_same_query!(
    referral_codes,
    referral_owners,
    referral_redemptions,
);
