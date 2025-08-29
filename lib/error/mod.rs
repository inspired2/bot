pub enum Error {
    Connection,
    File
}
impl Error {
    fn new()-> Self {
        Error::Connection
    }
}
