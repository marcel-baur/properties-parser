# Properties Parser
** Use versions 0.2.3 or upwards. Previous versions have been yanke due to bugs and incomplete parsings. Usage is still the same**
A library to parse `.properties` files.

Use the published `fetch_file` function to parse a file from the given path.
On success, it returns a `Vec<Entry>`, which contains all entries in the provided
file.

An `Entry` provides the following data structure:
```
pub type Entry = (Key, Value);
pub type Key = Vec<String>;

pub enum Value {
    Integer(i64),
    Null,
    String(String),
}

```
