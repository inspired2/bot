use bot::{error::Error, traits::bot::Bot};

use crate::max::{api::{Api, Message}, config::Config};

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
