table! {
    accounts (id) {
        id -> Text,
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
        note_id -> Text,
        account_id -> Text,
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
    }
}

table! {
    reset_tokens (account_id) {
        account_id -> Text,
        reset_token -> Text,
        expires_at -> Timestamptz,
    }
}

table! {
    verification_tokens (account_id) {
        account_id -> Text,
        verification_token -> Text,
        expires_at -> Timestamptz,
    }
}

joinable!(notes -> accounts (account_id));
joinable!(reset_tokens -> accounts (account_id));
joinable!(verification_tokens -> accounts (account_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    notes,
    reset_tokens,
    verification_tokens,
);
