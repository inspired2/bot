pub trait Bot<E: BotError, M: BotMessage, A: BotApi<E,M>> {
    fn from_config(config: impl IntoConfig) -> Result<Self, E>
    where
        Self: Sized;
    fn with_api(self, api: A) -> Result<Self, E> where Self: Sized;
    fn run(self) -> impl std::future::Future<Output = Result<(), E>> + Send;
}

pub trait IntoConfig {
    fn into_config<C: BotConfig, E: BotError>(file: impl tokio::io::AsyncRead) -> Result<C, E>;
}

pub trait BotApi<E: BotError, M: BotMessage> {
    async fn get_messages(&self, message: M) -> Result<Vec<M>, E>;
}

pub trait BotConfig {}

pub trait BotError {}

pub trait BotMessage {}