use std::{env, marker::PhantomData};
use crate::structs::OpenAi;

#[derive(Debug)]
pub struct Token<T>(String, PhantomData<T>);

impl Token<OpenAi> {
    pub fn read_env() -> Option<Self> {
        let mut token = env::var("CHAI_OPENAI_API_KEY");
        if let Err(env::VarError::NotPresent) = token {
            token = env::var("OPENAI_API_KEY");
        }

        match token {
            Ok(token) => Some(Self(token, PhantomData)),
            _ => None
        }
    }
}
