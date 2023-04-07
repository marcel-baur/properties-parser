use crate::types::Span;

#[derive(Debug)]
pub enum LibError {
    LexError(String, Span),
    ParserError(String),
    IOError(std::io::Error),
}

impl From<std::io::Error> for LibError {
    fn from(x: std::io::Error) -> Self {
        LibError::IOError(x)
    }
}
