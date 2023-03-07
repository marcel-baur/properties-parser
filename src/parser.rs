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
    Space,
}

#[derive(Debug, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Span {
        Span { start, end }
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
        TokenWrapper {
            content: Token::Garbage,
            span,
        }
    }
}

/// Represents an entry of a `.properties` file
///
/// # Example
/// this.is.an = example
pub type Entry = (Key, Value);

/// Represents a possible Value of a `.properties` file entry.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    // Real(String),
    Integer(i64),
    Null,
    String(String),
}

pub type Key = Vec<String>;

enum ParsingState {
    Key,
    Value,
}

fn update_current_value(current_value: &Value, character: char) -> Value {
    match current_value {
        Value::Null => return Value::String(character.to_string()),
        Value::String(s) => {
            return Value::String({
                let ref mut this = s.clone();
                this.push(character);
                this.to_string()
            })
        }
        _ => {
            // TODO: fix this for other types. Not used for now
            return Value::Null;
        }
    }
}

/// Returns the result of parsing a lexed `.properties` file.
///
/// # Arguments
///
/// * `tokens` - A `Vec` of `TokenWrapper`s representing a lexed file
pub fn parse_file(tokens: Vec<TokenWrapper>) -> Result<Vec<Entry>, LibError> {
    let mut props = Vec::new();
    let mut current_key = Vec::new();
    let mut current_value = Value::Null;
    let mut parsing_state = ParsingState::Key;
    let mut is_comment_line = false;
    for token in tokens {
        if is_comment_line {
            match token.content {
                Token::Eol => {
                    is_comment_line = false;
                    continue;
                }
                _ => continue,
            };
        }
        match token.content {
            Token::String(val) => {
                match parsing_state {
                    ParsingState::Key => {
                        current_key.push(val);
                    }
                    ParsingState::Value => {
                        match current_value {
                            Value::Null => current_value = Value::String(val),
                            Value::String(s) => {
                                current_value = Value::String({
                                    let this = &mut s.clone();
                                    this.push_str(&val);
                                    this.to_string()
                                })
                            }
                            _ => {}
                        };
                    }
                };
            }
            Token::Number(val) => {
                match parsing_state {
                    ParsingState::Key => {
                        current_key.push(val.to_string());
                    }
                    ParsingState::Value => {
                        current_value = Value::Integer(val);
                    }
                };
            }
            Token::CommentSign => {
                is_comment_line = true;
                continue;
            }
            Token::Equals => {
                match parsing_state {
                    ParsingState::Key => parsing_state = ParsingState::Value,
                    ParsingState::Value => {
                        current_value = update_current_value(&current_value, '=');
                    }
                };
            }
            Token::Dot => {
                match parsing_state {
                    ParsingState::Key => continue,
                    ParsingState::Value => {
                        current_value = update_current_value(&current_value, '.');
                    }
                };
            }
            Token::Eol => {
                parsing_state = ParsingState::Key;
                props.push((current_key.clone(), current_value.clone()));
                // TODO: maybe reset key, value
                current_key = Vec::new();
                current_value = Value::Null;
            }
            Token::Eof => {
                if !current_key.is_empty() {
                    props.push((current_key.clone(), current_value.clone()));
                }
                return Ok(props)
            },
            Token::Space => continue,
            Token::Garbage => {
                println!("Bad token, skipping it");
                continue;
            }
        };
    }

    return Ok(props);
}
