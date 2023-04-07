use crate::error::LibError;
use crate::types::{Token, TokenWrapper, Span};

fn lower_bound_zero(num: &usize) -> usize {
    if *num > 0 {
        return num - 1;
    }
    return 0;
}

pub fn lex(bytes: Vec<u8>) -> Result<Vec<TokenWrapper>, LibError> {
    let mut index = 0;
    let mut output = Vec::new();

    while index < bytes.len() {
        let c = bytes[index];
        let prev = bytes[lower_bound_zero(&index)];
        match c {
            b'#' => {
                let start = index;
                index += 1;
                if prev != b'\n' && index != 1 {
                    match handle_regular_character(&bytes, &mut index, &mut output) {
                        Ok(_) => continue,
                        Err(e) => return Err(e),
                    };
                }
                output.push(TokenWrapper::new(
                    Token::CommentSign,
                    Span::new(start, start + 1),
                ));
            }
            b'.' => {
                let start = index;
                index += 1;
                output.push(TokenWrapper::new(Token::Dot, Span::new(start, start + 1)));
            }
            b'=' => {
                let start = index;
                index += 1;
                output.push(TokenWrapper::new(
                    Token::Equals,
                    Span::new(start, start + 1),
                ));
            }
            b'\n' => {
                let start = index;
                index += 1;
                output.push(TokenWrapper::new(Token::Eol, Span::new(start, start + 1)));
            }
            b' ' => {
                let start = index;
                index += 1;
                output.push(TokenWrapper::new(Token::Space, Span::new(start, start + 1)));
            }
            _ => {
                // TODO: consume token
                match handle_regular_character(&bytes, &mut index, &mut output) {
                    Ok(_) => continue,
                    Err(e) => return Err(e),
                };
            }
        };
    }
    output.push(TokenWrapper {
        content: Token::Eof,
        span: Span::new(index, index),
    });
    return Ok(output);
}

fn handle_regular_character(
    bytes: &[u8],
    index: &mut usize,
    output: &mut Vec<TokenWrapper>,
) -> Result<(), LibError> {
    match lex_item(bytes, index) {
        Ok(t) => output.push(t),
        Err(e) => {
            match e {
                LibError::LexError(_m, s) => output.push(TokenWrapper::ukn(s)),
                _ => {
                    return Err(e);
                }
            };
        }
    };
    Ok(())
}

fn lex_item(bytes: &[u8], index: &mut usize) -> Result<TokenWrapper, LibError> {
    if bytes[*index].is_ascii() {
        let start = *index;
        *index += 1;

        while *index < bytes.len()
            && (bytes[*index].is_ascii_alphanumeric() || bytes[*index] == b'#')
        {
            *index += 1;
        }
        let str = String::from_utf8_lossy(&bytes[start..*index]);
        // TODO: check Token Type for designator
        return Ok(TokenWrapper::new(
            Token::String(str.to_string()),
            Span::new(start, *index),
        ));
    }

    if bytes[*index].is_ascii_digit() {
        let start = *index;
        while *index < bytes.len() && bytes[*index].is_ascii_digit() {
            *index += 1;
        }

        let str = String::from_utf8_lossy(&bytes[start..*index]);
        let number: Result<i64, _> = str.parse();

        return match number {
            Ok(num) => Ok(TokenWrapper::new(
                Token::Number(num),
                Span::new(start, *index),
            )),
            Err(_) => Err(LibError::LexError(
                "could not parse to number".to_string(),
                Span::new(start, *index),
            )),
        };
    }
    if bytes[*index].is_ascii_alphabetic() {
        let start = *index;
        *index += 1;

        while *index < bytes.len() && (bytes[*index].is_ascii_alphanumeric()) {
            *index += 1;
        }
        let str = String::from_utf8_lossy(&bytes[start..*index]);
        // TODO: check Token Type for designator
        return Ok(TokenWrapper::new(
            Token::String(str.to_string()),
            Span::new(start, *index),
        ));
    }
    let span = Span::new(*index, *index + 1);
    // error = error.or(Some(LibError::ParserError("unkown character".to_string(), span)));
    *index += 1;
    Err(LibError::LexError("unknown character".to_string(), span))
}
