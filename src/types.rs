#[derive(Clone, Debug)]
pub struct ContentNode {
    pub node_type: NodeType,
    pub name: String,
    pub is_root: bool,
}

#[derive(Clone, Debug)]
pub enum NodeType {
    Internal(Vec<ContentNode>),
    
    Leaf(String),
    
    Unexpected,
}

impl NodeType {
    fn get_content(&mut self) -> Option<&mut Vec<ContentNode>> {
        match self {
            Self::Internal(v) => Some(v),
            _ => None
        }
    }
}

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
