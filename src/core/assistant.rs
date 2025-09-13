use reqwest::Response;

#[derive(Debug, thiserror::Error)]
pub enum AssistantError {
    #[error("Error with request: {0}")]
    Request(#[from] reqwest::Error)
}

pub trait Assistant {
    fn send(&self, message: String) -> impl Future<Output = Result<Response, AssistantError>>;
}
