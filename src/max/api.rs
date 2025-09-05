use bot::{error::Error, traits::bot::{BotApi, BotMessage}};


pub struct Api {}
impl BotApi<Error, Message> for Api {
    async fn get_messages(&self, message: Message) -> Result<Vec<Message>, Error> {
        todo!()
    }
    
    async fn from_file(path: std::path::PathBuf) -> Result<Self, Error>
        where Self: Sized {
        todo!()
    }
}

pub enum Message {}
impl BotMessage for Message {}