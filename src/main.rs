mod max;

use max::bot::MaxBot;
use bot::traits::bot::{Bot, BotApi, BotConfig, BotState};
use bot::args::CliArgs;
use clap::Parser;
use crate::max::api::Api;
use crate::max::config::Config;
use crate::max::state::State;
use bot::error::Error;

use tokio;

#[tokio::main]
async fn main() -> Result<(), Error> {

    let cli_args = get_cli_args();
    let app_config = Config::from_file(cli_args.get_config_path())
        .await?;
    
    let api_path= app_config.get_api_path();
    let api = Api::from_file(api_path).await?;

    let state = State::init();

    let bot = MaxBot::from_config(app_config)?
        .with_api(api)?
        .with_state(state)?;
    
    bot.run().await?;

    Ok(())
}

fn get_cli_args() -> CliArgs {
    let cwd = std::env::current_dir().unwrap();
    let mut args = CliArgs::parse();
    args.set_cwd(cwd);
    args
}