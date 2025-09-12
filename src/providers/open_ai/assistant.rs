use crate::core::assistant::{Assistant, AssistantError, HasApi};
use crate::core::token::{HasToken, Token, TokenError};
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
    async fn send() -> Result<Response, AssistantError> {
        unimplemented!("do later")
    }
}

impl HasToken for OpenAi {
    const VAR: &'static str = "CHAI_OPENAI_API_KEY";
}

impl HasApi for OpenAi {
    const BASE_URL: &'static str = "https://api.openai.com/v1/";
}
