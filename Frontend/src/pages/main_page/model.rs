use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct FileUploadData {
    pub id: i32,
    pub secret: String,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
