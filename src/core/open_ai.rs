use crate::core::HasToken;

#[derive(Debug)]
pub struct OpenAi {}

impl HasToken for OpenAi {
    const VAR: &'static str = "CHAI_OPENAI_API_KEY";
}
