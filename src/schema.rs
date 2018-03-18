// generate this file's content with `diesel print-schema`

table! {
    github_accounts (id) {
        id -> Int4,
        user_id -> Int4,
        access_token -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        uuid -> Uuid,
        username -> Text,
        role -> Nullable<Text>,
        email -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(github_accounts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    github_accounts,
    users,
);
