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

