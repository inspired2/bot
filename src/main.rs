mod max_bot;
use bot::traits::bot::Bot;
use max_bot::{MaxBot, Api, Config};

use crate::tokio::io::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let file = tokio::fs::File::open("./config.json").await.ok().unwrap();
    //let config = MaxBot::from_config(file).await.unwrap();
    Ok(())
}
