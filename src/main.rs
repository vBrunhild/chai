mod cli;
mod core;
mod providers;

use reqwest::Client;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::{env, fs, sync::LazyLock};

pub static CLIENT: LazyLock<Client> = LazyLock::new(Client::new);

pub static DB: LazyLock<SqlitePool> = LazyLock::new(|| {
    let db_path = if cfg!(debug_assertions) {
        "./chai.db".into()
    } else {
        env::home_dir()
            .expect("has a home directory")
            .join("chai")
            .join("chai.db")
    };

    fs::create_dir_all(db_path.parent().expect("has parent"))
        .expect("creates parent dirs");

    let url = format!("sqlite://{}", db_path.display());
    SqlitePoolOptions::new()
        .connect_lazy(url.as_str())
        .expect("creates / accesses database")
});

#[tokio::main]
async fn main() {
    let provider = providers::open_ai::OpenAi::new().unwrap();
}
