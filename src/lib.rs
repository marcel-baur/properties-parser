use error::LibError;
use parser::Entry;

mod error;
mod lexer;
mod parser;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn fetch_file(fname: &str) -> Result<Vec<Entry>, LibError>{
    let content = std::fs::read(fname)?;
    let lexed = lexer::lex(content);
    return parser::parse_file(lexed.unwrap());
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
