use crate::config::Config;
use crate::error::Error;

pub trait Bot {
    fn from_config(config: impl IntoConfig) -> Result<Self, Error>
    where
        Self: Sized;
    async fn run(&mut self) -> Result<(), Error>;
}

pub trait IntoConfig {
    fn into_config(file: impl tokio::io::AsyncRead) -> Result<Config, Error>;
}
