use crate::traits::bot::BotError;
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}
impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Error {
            kind
        }
    }
    pub fn custom(s: String) -> Self {
        Error {
            kind: ErrorKind::custom(s)
        }
    }
}
#[derive(Debug)]
pub enum ErrorKind {
    io(tokio::io::Error),
    custom(String)
}
impl From<tokio::io::Error> for Error {
    fn from(value: tokio::io::Error) -> Self {
        Error {
            kind: ErrorKind::io(value)
        }
    }
}
impl BotError for Error {}