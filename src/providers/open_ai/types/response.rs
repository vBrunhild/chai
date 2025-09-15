use crate::providers::open_ai::types::Output;

#[derive(Debug, serde::Deserialize)]
pub struct Response {
    pub id: String,
    pub created_at: u32,
    pub status: ResponseStatus,
    pub error: Option<Error>,
    pub output: Vec<Output>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseStatus {
    Cancelled,
    Completed,
    Failed,
    InProgress,
    Incomplete,
    Queued,
}

#[derive(Debug, serde::Deserialize)]
pub struct Error {
    pub code: String,
    pub message: String,
}
