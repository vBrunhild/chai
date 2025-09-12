use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "chai")]
#[command(about = "Send message", long_about = None)]
pub struct Cli {
    message: Vec<String>,
}
