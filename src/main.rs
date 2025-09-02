mod max_bot;

use bot::traits::bot::{Bot, BotApi, BotConfig};
use bot::args::CliArgs;
use clap::Parser;
use max_bot::{Api, Config};
use bot::error::Error;

use tokio;

#[tokio::main]
async fn main() -> Result<(), Error> {

    let cli_args = get_cli_args();
    let app_config = Config::from_file(cli_args.get_config_path())
        .await?;

    let api = Api::new();

    let bot = max_bot::MaxBot::from_config(app_config)?
        .with_api(api)?;
    bot.run().await?;

    Ok(())
}

fn get_cli_args() -> CliArgs {
    let cwd = std::env::current_dir().unwrap();
    let mut args = CliArgs::parse();
    args.set_cwd(cwd);
    args
}