#[derive(Debug, serde::Deserialize)]
pub enum Content {
    Output(String),
    Refusal { refusal: String },
}
