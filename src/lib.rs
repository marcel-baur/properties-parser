use error::LibError;
use parser::{TokenWrapper, Token, Span};

mod error;
mod parser;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn fetch_file(fname: &str) -> Result<(), LibError>{
    let content = std::fs::read(fname)?;
    Ok(())
}

fn parse(bytes: Vec<u8>) {
    let mut index = 0;
    // let mut error = None;
    let mut output = Vec::new();

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
            _ => {
                // TODO: consume token
                
            }
        }
        if c == b'#' {
            let start = index;
            index += 1;
            output.push(TokenWrapper::new(Token::CommentSign, Span::new(start, start + 1))  );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
