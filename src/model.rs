use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Identifiable, AsChangeset, Debug, PartialEq, Serialize, Deserialize, Default, Clone,
)]
#[table_name = "file_uploads"]
pub struct FileUpload {
    pub id: i32,
    pub secret: String,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, AsChangeset, Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
#[table_name = "file_uploads"]
#[changeset_options(treat_none_as_null = "true")]
pub struct FileUploadNoID {
    pub secret: String,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(
    Queryable, Identifiable, AsChangeset, Debug, PartialEq, Serialize, Deserialize, Default, Clone,
)]
#[table_name = "sessions"]
pub struct Session {
    pub id: i32,
    pub session_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub expires_at: chrono::NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
#[table_name = "sessions"]
pub struct SessionNoID {
    pub session_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub expires_at: chrono::NaiveDateTime,
}
