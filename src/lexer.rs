use crate::error::LibError;
use crate::parser::{TokenWrapper, Token, Span};

pub fn lex(bytes: Vec<u8>) -> (Vec<TokenWrapper>, Option<LibError>) {
    let mut index = 0;
    // let mut error = None;
    let mut output = Vec::new();
    let mut error: Option<LibError> = None;

    while index < bytes.len() {
        let c = bytes[index];
        match c {
            b'#' => {
                let start = index;
                index += 1;
                output.push(TokenWrapper::new(Token::CommentSign, Span::new(start, start + 1)));
            }
            b'.' => {
                let start = index;
                index += 1;
                output.push(TokenWrapper::new(Token::Dot, Span::new(start, start + 1)));
            }
            b'=' => {
                let start = index;
                index += 1;
                output.push(TokenWrapper::new(Token::Equals, Span::new(start, start + 1)));
            }
            b'\n' => {
                let start = index;
                index += 1;
                output.push(
                    TokenWrapper::new(Token::Eol, Span::new(start, start + 1))
                );
            }
            _ => {
                // TODO: consume token
                let (token, err) = lex_item(&bytes, &mut index);
                error = error.or(err);
                output.push(token);
            }
        }
    }
    output.push(TokenWrapper { content: Token::Eof, span: Span::new(index, index) });
    return (output, error);
}

fn lex_item(bytes: &[u8], index: &mut usize) -> (TokenWrapper, Option<LibError>) {
    let mut error = None;

    if bytes[*index].is_ascii_digit() {
        let start = *index;
        while *index < bytes.len() && bytes[*index].is_ascii_digit() {
            *index += 1;
        }

        let str = String::from_utf8_lossy(&bytes[start..*index]);
        let number: Result<i64, _> = str.parse();

        return match number {
            Ok(num) => (TokenWrapper::new(Token::Number(num), Span::new(start, *index)), None),
            Err(_) => (TokenWrapper::ukn(Span::new(start, *index)), Some(LibError::ParserError("could not parse to number".to_string(), Span::new(start, *index))))
        };
    } else if bytes[*index].is_ascii_alphabetic() {
        let start = *index;
        *index += 1;
        
        while *index < bytes.len() && (bytes[*index].is_ascii_alphanumeric()) {
            *index += 1;
        }
        let str = String::from_utf8_lossy(&bytes[start..*index]);
        // TODO: check Token Type for designator
        return (TokenWrapper::new(Token::String(str.to_string()), Span::new(start, *index)), error);
    } else {
        let span = Span::new(*index, *index + 1);
        error = error.or(Some(LibError::ParserError("unkown character".to_string(), span)));
        *index += 1;
        return (TokenWrapper::ukn(span), error);
    }
}

