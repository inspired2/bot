use std::path::PathBuf;

use bot::{error::Error, traits::bot::BotConfig};

pub struct Config;

impl BotConfig<Error> for Config {
    async fn from_file(path: PathBuf) -> Result<Self, Error>
    where Self: Sized {
        todo!()
    }
    
    fn get_api_path(&self) -> PathBuf {
        todo!()
    }
    

}
