// @generated automatically by Diesel CLI.

diesel::table! {
    file_uploads (id) {
        id -> Int4,
        secret -> Varchar,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    sessions (id) {
        id -> Int4,
        session_id -> Varchar,
        created_at -> Timestamp,
        expires_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    file_uploads,
    sessions,
);
