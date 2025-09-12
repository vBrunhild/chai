use reqwest::Response;

#[derive(Debug, thiserror::Error)]
pub enum AssistantError {
    #[error("Error with request: {0}")]
    Request(#[from] reqwest::Error)
}

pub trait Assistant {
    fn send() -> impl Future<Output = Result<Response, AssistantError>>;
}

pub trait HasApi {
    const BASE_URL: &'static str;
}
