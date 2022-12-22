#[derive(Debug)]
pub enum LibError {
    ParserError(String, Span),
    IOError(std::io::Error)
}

impl From<std::io::Error> for LibError {
    fn from(x: std::io::Error) -> Self {
        LibError::IOError(x)
    }
}
