use crate::error::LibError;

#[derive(Debug)]
pub enum Type {
    Number,
    String,
    Void,
}

pub struct Variable {
    pub name: String,
    pub ty: Type,
}

impl Variable {
    pub fn new() -> Self {
        Self { 
            name: String::new(),
            ty: Type::Void,
        }
    }
}


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

pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Span {
        Span { start, end}
    }
}

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

pub enum ContextEntry {
    StringValue(String),
    NumberValue(i64),
    Nested(Box<ContextEntry>),
}

pub struct ParsedFile {
    pub entries: Vec<ContextEntry>
}

impl ParsedFile {
    fn new() -> Self {
        Self {
            entries: Vec::new()
        }
    }
}

pub enum ParsingState {
    Key,
    Value,
}

pub fn parse_file(tokens: &[TokenWrapper]) -> Result<ParsedFile, LibError> {
    let mut index = 0;
    let mut parsed_file = ParsedFile::new();
    let mut parsing_state = ParsingState::Key;

    while index < tokens.len(){
        let token = &tokens[index];
        // TODO: differentiate between key and value during lexing
        match token {
            TokenWrapper { content: Token::Eol, .. } => { 
                index += 1; 
                parsing_state = ParsingState::Key;
            }
            TokenWrapper { content: Token::Equals, .. } => {
                // This might not be necessary because it's done in parse_entry
                index += 1;
                parsing_state = ParsingState::Value;
            }
            TokenWrapper { content: Token::String(value), span, } => {
                // This handles the entire entry until the Eol
                let content_entry = parse_entry(tokens, &mut index);
            }
            TokenWrapper { content: Token::Eof, .. } => { break; }
            
            TokenWrapper { span, .. } => {
                return Err(LibError::ParserError("unexpected token".to_string(), span));
            }
        }
    }

    return Ok(parsed_file);
}

fn parse_entry(tokens: &[TokenWrapper], index: &mut usize) -> Option<ContextEntry> {
    todo!()
}
