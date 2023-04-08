# Properties Parser
**Use versions 0.2.3 or upwards. Previous versions have been yanked due to bugs and incomplete parsings. Usage is still the same**

A library to parse `.properties` files.

Types are defined in the `types` module.

Use the published `parse_file` function to parse a file from the given path.
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
