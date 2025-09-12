use crate::providers::open_ai::types::Input;

#[derive(Debug, serde::Serialize)]
pub enum Data {
    Input(Input)
}
