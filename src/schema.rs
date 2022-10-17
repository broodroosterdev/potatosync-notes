table! {
    blobs (id, account_id) {
        id -> Text,
        account_id -> Text,
        content -> Text,
        last_changed -> Int8,
    }
}

allow_tables_to_appear_in_same_query!(
    blobs,
);
