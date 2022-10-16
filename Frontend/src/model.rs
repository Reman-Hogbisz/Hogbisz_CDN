use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ServerResponse<T> {
    pub status: u16,
    pub message: String,
    pub data: Option<T>,
}
