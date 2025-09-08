use bot::{error::Error, traits::bot::Bot};

use crate::max::{api::{Api, Message}, config::Config, state::State};

pub struct MaxBot {
    config: Option<Config>,
    api: Option<Api>,
    state: Option<State>
}

impl Bot<Error, Message, Api, Config, State> for MaxBot {
    fn from_config(config: Config) -> Result<Self, Error>
    where
        Self: Sized {
        let bot = MaxBot {
            config: Some(config.into()),
            api: None,
            state: None
        };
        Ok(bot)
    }

    fn with_api(mut self, api: Api) -> Result<Self, Error> where Self: Sized {
        self.api = Some(api);
        Ok(self)
    }
    fn with_state(mut self, s: State) -> Result<Self, Error> where Self: Sized {
        self.state = Some(s);
        Ok(self)
    }
    async fn run(self) -> Result<(), Error> {
        assert!(&self.state.as_ref().is_some(), "state is not initialized. call 'with_state(state)' before 'run()'");
        assert!(&self.api.as_ref().is_some(), "api is not initialized. call 'with_api(api)' before 'run()'");
        Ok(())
    }   
}
