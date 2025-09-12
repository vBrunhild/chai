use crate::providers::open_ai::types::Message;

#[derive(Debug, serde::Deserialize)]
pub enum Output {
    Message(Message),
}
