use std::path::PathBuf;

pub trait Bot<E: BotError, M: BotMessage, A: BotApi<E,M>, C: BotConfig<E>, S: BotState> {
    fn from_config(config: C) -> Result<Self, E>
    where
        Self: Sized;
    fn with_api(self, api: A) -> Result<Self, E> where Self: Sized;
    fn with_state(self, state: S) -> Result<Self, E> where Self: Sized;
    fn run(self) -> impl std::future::Future<Output = Result<(), E>> + Send;
}

pub trait IntoConfig {
    fn into_config<C: BotConfig<E>, E: BotError>(file: impl tokio::io::AsyncRead) -> Result<C, E>;
}

pub trait BotApi<E: BotError, M: BotMessage> {
    async fn from_file(path: PathBuf) -> Result<Self, E>
        where Self: Sized;
    fn get_messages(&self, message: M) -> impl std::future::Future<Output = Result<Vec<M>, E>> + Send;
}

pub trait BotConfig<E: BotError> {
    fn from_file(path: PathBuf) -> impl std::future::Future<Output = Result<Self, E>> + Send
        where Self: Sized;
fn get_api_path(&self) -> PathBuf;

}

pub trait BotError {}
pub trait BotMessage {}

pub trait BotState: Sync {
    fn init() -> Self
    where Self: Sized;
}