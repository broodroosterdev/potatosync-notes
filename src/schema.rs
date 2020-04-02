table! {
    accounts (id) {
        id -> Int4,
        created_at -> Text,
        updated_at -> Nullable<Text>,
        deleted_at -> Nullable<Text>,
        email -> Text,
        username -> Text,
        password -> Text,
        image_url -> Text,
        password_identifier -> Text,
        verified -> Bool,
    }
}

table! {
    notes (note_id, account_id) {
        note_id -> Int4,
        account_id -> Int4,
        title -> Text,
        content -> Text,
        image_url -> Nullable<Text>,
        list_parse_string -> Nullable<Text>,
        reminders -> Nullable<Text>,
        date -> Text,
        color -> Int4,
        hide_content -> Bool,
        is_deleted -> Bool,
        is_archived -> Bool,
        is_list -> Bool,
        is_starred -> Bool,
        pin -> Nullable<Text>,
        password -> Nullable<Text>,
    }
}

table! {
    tokens (account_id) {
        account_id -> Int4,
        token -> Text,
    }
}

joinable!(notes -> accounts (account_id));
joinable!(tokens -> accounts (account_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    notes,
    tokens,
);
