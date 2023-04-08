use error::LibError;
use types::Entry;

mod error;
mod lexer;
mod parser;
pub mod types;

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
#[deprecated]
pub fn fetch_file(fname: &str) -> Result<Vec<Entry>, LibError> {
    let content = std::fs::read(fname)?;
    let lexed = lexer::lex(content);
    return parser::parse_file(lexed.unwrap());
}

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
pub fn parse_file(fname: &str) -> Result<Vec<Entry>, LibError> {
    let content = std::fs::read(fname)?;
    let lexed = lexer::lex(content);
    return parser::parse_file(lexed.unwrap());
}

#[cfg(test)]
mod tests {
    use crate::types::Value;

    use super::*;

    fn generate_result_prop(key: String, value: String) -> types::Entry {
        return (
            key.split(".").map(|e| e.to_string()).collect(),
            Value::String(value),
        );
    }

    #[test]
    fn parses_basic_entry() {
        let obj = "this.is.a = prop".as_bytes();
        let expected = vec![generate_result_prop(
            "this.is.a".to_string(),
            "prop".to_string(),
        )];
        let lexed = lexer::lex(obj.to_vec()).unwrap();
        let parsed = parser::parse_file(lexed);
        assert_eq!(parsed.unwrap(), expected);
    }

    #[test]
    fn parses_mixed_entry() {
        let obj = "this.is.a = m1x3d3ntry".as_bytes();
        let expected = vec![generate_result_prop(
            "this.is.a".to_string(),
            "m1x3d3ntry".to_string(),
        )];
        let lexed = lexer::lex(obj.to_vec()).unwrap();
        let parsed = parser::parse_file(lexed);
        assert_eq!(parsed.unwrap(), expected);
    }

    #[test]
    fn parses_url_entry() {
        let obj = "this.is.a = http://www.google.com/abcd?prop=prop".as_bytes();
        let expected = vec![generate_result_prop(
            "this.is.a".to_string(),
            "http://www.google.com/abcd?prop=prop".to_string(),
        )];
        let lexed = lexer::lex(obj.to_vec()).unwrap();
        let parsed = parser::parse_file(lexed);
        assert_eq!(parsed.unwrap(), expected);
    }

    #[test]
    fn parses_entry_with_hash() {
        let obj = "this.is.a = entry#has#hash".as_bytes();
        let expected = vec![generate_result_prop(
            "this.is.a".to_string(),
            "entry#has#hash".to_string(),
        )];
        let lexed = lexer::lex(obj.to_vec()).unwrap();
        let parsed = parser::parse_file(lexed);
        assert_eq!(parsed.unwrap(), expected);
    }

    #[test]
    fn correctly_parses_basic_file() {
        let res = parse_file("res/demo.properties").unwrap();
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
            (
                vec!["has".to_string(), "a".to_string(), "hash".to_string()],
                Value::String("has#hash".to_string()),
            ),
        ];
        // let expect = [(ex_vec, Value::String("prop".to_string()))];
        assert_eq!(res, exp);
    }
}
