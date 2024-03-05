
table! {
    transactions (id) {
        id -> Int4,
        client_id -> Int4,
        value -> Int4,
        transaction_type -> Text,
        description -> Text,
        created_at -> Timestamp,
    }
}
