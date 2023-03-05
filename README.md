# Properties Parser

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
