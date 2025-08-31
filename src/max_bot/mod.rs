use std::path::PathBuf;

use bot::error::Error;
use bot::traits::bot::{Bot, BotConfig, BotApi, BotMessage};
pub struct MaxBot {
    config: Option<Config>,
    api: Option<Api>,
}

impl Bot<Error, Message, Api, Config> for MaxBot {
    fn from_config(config: Config) -> Result<Self, Error>
    where
        Self: Sized {
        let bot = MaxBot {
            config: Some(config.into()),
            api: None
        };
        Ok(bot)
    }

    fn with_api(mut self, api: Api) -> Result<Self, Error> where Self: Sized {
        self.api = Some(api);
        Ok(self)
    }

    async fn run(self) -> Result<(), Error> {
        todo!()
    }
}
pub struct Config;

impl BotConfig<Error> for Config {
    async fn from_file(path: PathBuf) -> Result<Self, Error>
    where Self: Sized {
        todo!()
    }
}

pub struct Api {}
impl BotApi<Error, Message> for Api {
    async fn get_messages(&self, message: Message) -> Result<Vec<Message>, Error> {
        todo!()
    }
}

enum Message {}
impl BotMessage for Message {}