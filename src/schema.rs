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
