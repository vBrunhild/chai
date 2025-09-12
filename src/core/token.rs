use std::{env, fmt::Debug, marker::PhantomData};

pub struct Token<T>(String, PhantomData<T>);

impl<T> Token<T> {
    const fn new(token: String) -> Self {
        Self(token, PhantomData)
    }
}

impl<T: Debug> Debug for Token<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f
            .debug_tuple("Token")
            .field(&"****")
            .field(&self.1)
            .finish()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TokenError {
    #[error("Token is not present: {0}")]
    NotPresent(&'static str),
    #[error("Token is invalid")]
    NotUnicode,
}

pub trait HasToken: Sized {
    const VAR: &'static str;

    fn get_token() -> Result<Token<Self>, TokenError> {
        match env::var(Self::VAR) {
            Ok(token) => Ok(Token::new(token)),
            Err(env::VarError::NotPresent) => Err(TokenError::NotPresent(Self::VAR)),
            Err(env::VarError::NotUnicode(_)) => Err(TokenError::NotUnicode),
        }
    }
}
