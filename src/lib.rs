use error::LibError;

mod error;
mod lexer;
mod parser;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn fetch_file(fname: &str) -> Result<(), LibError>{
    let content = std::fs::read(fname)?;
    let lexed = lexer::lex(content);
    println!("{:?}", lexed);
    Ok(())
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
