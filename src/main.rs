mod structs;
use crate::structs::Token;

#[tokio::main]
async fn main() {
    let token = Token::read_env();
    dbg!(token);
}
