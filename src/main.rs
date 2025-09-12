mod core;
mod cli;

use std::sync::LazyLock;
use reqwest::Client;

pub static CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::new()
});

#[tokio::main]
async fn main() {
}
