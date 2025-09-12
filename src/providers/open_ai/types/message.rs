use crate::providers::open_ai::types::{Content, Status};

#[derive(Debug, serde::Deserialize)]
pub struct Message {
    pub id: String,
    pub status: Status,
    pub content: Vec<Content>,
}
