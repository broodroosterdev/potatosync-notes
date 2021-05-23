table! {
    notes (id, account_id) {
        id -> Text,
        account_id -> Text,
        content -> Text,
        last_changed -> Int8,
    }
}

table! {
    tags (id, account_id) {
        id -> Text,
        account_id -> Text,
        content -> Text,
        last_changed -> Int8,
    }
}

allow_tables_to_appear_in_same_query!(
    notes,
    tags,
);
