mod core;

use clap::Parser;
use core::{HasToken, OpenAi};

#[derive(Debug, Parser)]
#[command(name = "chai")]
#[command(about = "Send message", long_about = None)]
struct Cli {
    message: Vec<String>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let token = OpenAi::get_token();
    dbg!(token);
    println!("{}", cli.message.join(" "));
}
