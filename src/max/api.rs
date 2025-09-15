use std::collections::HashMap;

use bot::{error::Error, traits::bot::{BotApi, BotMessage}};

#[derive(Clone)]
pub struct Api {
    http: Req
    
}
impl BotApi<Error, Message> for Api {
    async fn get_messages(&self) -> Result<Vec<Message>, Error> {
        todo!()
    }
    
    async fn from_file(path: std::path::PathBuf) -> Result<Self, Error>
        where Self: Sized {
        todo!()
    }
}

pub enum Message {}
impl BotMessage for Message {}

#[derive(Clone)]
struct Req {
    inner: HashMap<Request, String>
}

#[derive(Clone)]
enum Request {
    GetMessages
}