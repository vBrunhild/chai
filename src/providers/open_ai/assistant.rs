use crate::CLIENT;
use crate::core::assistant::{Assistant, AssistantError};
use crate::core::token::{HasApi, Token, TokenError};
use crate::providers::open_ai::types::{Body, BodyInput, Model, Role};
use reqwest::Response;

#[derive(Debug)]
pub struct OpenAi {
    token: Token<Self>,
}

impl OpenAi {
    pub fn new() -> Result<Self, TokenError> {
        Ok(Self {
            token: Self::get_token()?,
        })
    }
}

impl Assistant for OpenAi {
    async fn send(&self, message: String) -> Result<Response, AssistantError> {
        CLIENT
            .post(format!("{}{}", Self::BASE_URL, "responses"))
            .bearer_auth(&self.token)
            .json(&Body {
                model: Model::Gpt4oMini,
                instructions: None,
                reasoning: None,
                store: Some(false),
                input: vec![BodyInput::new(Role::User, message)],
            })
            .send()
            .await
            .map_err(AssistantError::from)
    }
}

impl HasApi for OpenAi {
    const BASE_URL: &'static str = "https://api.openai.com/v1/";
    const VAR: &'static str = "CHAI_OPENAI_API_KEY";
}
