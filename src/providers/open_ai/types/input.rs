use crate::providers::open_ai::types::Data;

#[derive(Debug, serde::Serialize)]
pub struct Input {
    pub object: String,
    pub data: Vec<Data>,
    pub first_id: String,
    pub last_id: String,
    pub has_more: bool,
}
