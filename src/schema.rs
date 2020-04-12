table! {
    accounts (id) {
        id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
        email -> Text,
        username -> Text,
        password -> Text,
        image_url -> Text,
        password_identifier -> Text,
        verified -> Bool,
        shared_prefs -> Text,
    }
}

table! {
    notes (note_id, account_id) {
        note_id -> Int4,
        account_id -> Int4,
        title -> Text,
        content -> Text,
        style_json -> Text,
        starred -> Bool,
        creation_date -> Timestamptz,
        last_modify_date -> Timestamptz,
        color -> Int4,
        images -> Text,
        list -> Bool,
        list_content -> Text,
        reminders -> Text,
        hide_content -> Bool,
        lock_note -> Bool,
        uses_biometrics -> Bool,
        deleted -> Bool,
        archived -> Bool,
        synced -> Bool,
    }
}

table! {
    tokens (account_id) {
        account_id -> Int4,
        token -> Text,
        created_at -> Text,
    }
}

joinable!(notes -> accounts (account_id));
joinable!(tokens -> accounts (account_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    notes,
    tokens,
);
