use linked_hash_map::LinkedHashMap;

use crate::error::LibError;

#[derive(Debug)]
pub enum Token {
    String(String),
    Number(i64),
    CommentSign, 
    Dot,
    Equals,
    Eol,
    Eof,
    Garbage,
}

#[derive(Debug, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Span {
        Span { start, end}
    }
}

#[derive(Debug)]
pub struct TokenWrapper {
    pub content: Token,
    pub span: Span,
}

impl TokenWrapper {
    pub fn new(content: Token, span: Span) -> TokenWrapper {
        TokenWrapper { content, span }
    }

    pub fn ukn(span: Span) -> TokenWrapper {
        TokenWrapper { content: Token::Garbage, span }
    }
}

pub type Hash = LinkedHashMap<Property, Property>;

pub type Entry = (Property, Property);

pub enum Property {
    Real(String),
    Integer(i64),
    Null,
    Hash(self::Hash),
}

enum ParsingState {
    Key,
    Value,
}

pub fn parse_file(tokens: Vec<TokenWrapper>) -> Result<Vec<Entry>, LibError> {
    let mut props = Vec::new();
    let mut current_key = Property::Null;
    let mut current_value = Property::Null;
    let mut parsing_state = ParsingState::Key;
    let mut is_comment_line = false;
    for token in tokens {
        if is_comment_line {
            match token.content {
                Token::Eol => {
                    is_comment_line = false;
                    continue;
                },
                _ => continue,
            };
        }
        match token.content {
            Token::String(val) => {
                match parsing_state {
                    ParsingState::Key => {},
                    ParsingState::Value => {},
                };
            },
            Token::Number(val) => {
                match parsing_state {
                    ParsingState::Key => {},
                    ParsingState::Value => {},
                };
            },
            Token::CommentSign => continue,
            Token::Dot => {
                match parsing_state {
                    ParsingState::Key => parsing_state = ParsingState::Value,
                    ParsingState::Value => parsing_state = ParsingState::Key,
                };
            },
            Token::Equals => continue,
            Token::Eol => {
                parsing_state = ParsingState::Key;
                props.push((current_key, current_value));
                // TODO: maybe reset key, value
            },
            Token::Eof => return Ok(props),
            Token::Garbage => return Err(LibError::ParserError("Bad item in TokenWrapper!".to_string())),
        };
    }

    return Ok(props);
}
