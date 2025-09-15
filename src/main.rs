mod cli;
mod core;
mod database;
mod providers;

use reqwest::Client;
use std::sync::LazyLock;

use crate::providers::open_ai;

pub static CLIENT: LazyLock<Client> = LazyLock::new(Client::new);
pub static DB: LazyLock<sqlx::Pool<sqlx::Sqlite>> = LazyLock::new(database::get);

#[tokio::main]
async fn main() {
    sqlx::migrate!()
        .run(&DB.clone())
        .await
        .expect("migrates db schemas");

    let provider = open_ai::OpenAi::new().unwrap();
}
