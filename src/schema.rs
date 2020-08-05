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
        tags -> Text,
        hide_content -> Bool,
        lock_note -> Bool,
        uses_biometrics -> Bool,
        deleted -> Bool,
        archived -> Bool,
    }
}

table! {
    settings (setting_key, account_id) {
        setting_key -> Text,
        account_id -> Text,
        setting_value -> Text,
        last_modify_date -> Timestamptz,
    }
}

table! {
    tags (id, account_id) {
        id -> Text,
        account_id -> Text,
        #[sql_name = "tag_name"]
        name -> Text,
        color -> Int4,
        last_modify_date -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    notes,
    settings,
    tags,
);
