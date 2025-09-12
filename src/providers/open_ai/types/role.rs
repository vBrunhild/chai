#[derive(Debug, serde::Serialize)]
pub enum Role {
    User,
    System,
    Developer,
}
