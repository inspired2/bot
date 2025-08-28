//use crate::traits::bot::{ Bot, BotError, BotConfig, BotApi, IntoConfig, BotMessage };
use bot::traits::bot::{Bot, BotError, BotConfig, BotApi, IntoConfig, BotMessage};
pub struct MaxBot {
    config: Option<Config>,
    api: Option<Api>,
}
impl Bot<Error, Message, Api> for MaxBot {
    fn from_config(config: impl IntoConfig) -> Result<Self, Error>
    where
        Self: Sized {
        todo!()
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

enum Error {}
impl BotError for Error {}

impl BotConfig for Config {}

pub struct Api {}
impl BotApi<Error, Message> for Api {
    async fn get_messages(&self, message: Message) -> Result<Vec<Message>, Error> {
        todo!()
    }
}

enum Message {}
impl BotMessage for Message {}