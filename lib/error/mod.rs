use crate::traits::bot::BotError;
pub struct Error {
    kind: ErrorKind,
}
pub enum ErrorKind {
    io(tokio::io::Error),
}
impl From<tokio::io::Error> for Error {
    fn from(value: tokio::io::Error) -> Self {
        Error {
            kind: ErrorKind::io(value)
        }
    }
}
impl BotError for Error {}