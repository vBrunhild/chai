use crate::providers::open_ai::types::Role;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Body {
    pub model: Model,
    pub instructions: Option<String>,
    pub reasoning: Option<Reasoning>,
    pub store: Option<bool>,
    pub input: Vec<BodyInput>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Model {
    #[serde(rename = "gpt-5")]
    Gpt5,
    #[serde(rename = "gpt-5-mini")]
    Gpt5Mini,
    #[serde(rename = "gpt-5-nano")]
    Gpt5Nano,
    #[serde(rename = "gpt-4.1")]
    Gpt4Dot1,
    #[serde(rename = "gpt-4o")]
    Gpt4o,
    #[serde(rename = "gpt-4o-mini")]
    Gpt4oMini,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Reasoning {
    pub effort: Option<ReasoningEffort>,
    pub summary: Option<ReasoningSummary>
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningEffort {
    Minimal,
    Low,
    Medium,
    High,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningSummary {
    Auto,
    Concise,
    Detailed,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BodyInput {
    #[serde(rename = "type")]
    type_: String,
    role: Role,
    content: String,
}

impl BodyInput {
    pub fn new(role: Role, content: String) -> Self {
        Self { type_: "message".into(), role, content }
    }
}
