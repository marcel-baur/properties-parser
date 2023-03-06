use error::LibError;
use parser::Entry;

mod error;
mod lexer;
pub mod parser;

/// Fetch a `.properties` file from a provided path and parse it
///
/// # Arguments
///
/// * `fname` - Path of the file that shall be parsed
///
/// # Returns
///
/// * on success: `Vec<Entry>` of the file
/// * on failiure: `LibError`
pub fn fetch_file(fname: &str) -> Result<Vec<Entry>, LibError> {
    let content = std::fs::read(fname)?;
    let lexed = lexer::lex(content);
    return parser::parse_file(lexed.unwrap());
}

#[cfg(test)]
mod tests {
    use crate::parser::Value;

    use super::*;

    #[test]
    fn correctly_parses_basic_file() {
        let res = fetch_file("res/demo.properties").unwrap();
        let mut ex_vec: Vec<String> = Vec::new();
        for el in ["this", "is", "a"] {
            ex_vec.push(el.to_string());
        }
        let exp = [
            (
                vec!["this".to_string(), "is".to_string(), "a".to_string()].to_vec(),
                Value::String("prop".to_string()),
            ),
            (
                vec!["url".to_string()].to_vec(),
                Value::String("http://asb.de".to_string()),
            ),
            (vec!["num".to_string()], Value::String("123".to_string())),
            (vec!["mix".to_string()], Value::String("ab33s".to_string())),
        ];
        // let expect = [(ex_vec, Value::String("prop".to_string()))];
        assert_eq!(res, exp);
    }
}
