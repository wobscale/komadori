// generate this file's content with `diesel print-schema`

table! {
    github_accounts (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        access_token -> Text,
        avatar -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        uuid -> Nullable<Uuid>,
        username -> Text,
        display_name -> Nullable<Text>,
        role -> Nullable<Text>,
        email -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(github_accounts -> users (user_id));
