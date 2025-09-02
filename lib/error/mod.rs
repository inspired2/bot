use crate::traits::bot::BotError;
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}
#[derive(Debug)]
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