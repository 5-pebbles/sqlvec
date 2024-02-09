# SqlVec

A generic container for vectors allowing for rusqlite operations.

`SqlVec` implements `ToSql` & `FromSql` storing values as `\u{F1}` delimited text.


```toml
[dependencies]
sqlvec = "0.0.1"
```

## Usage

Wrap a vector with `SqlVec` before passing to the database.

```rust
use sqlvec::SqlVec;

let values = SqlVec::new(vec!["one".to_string(), "two".to_string()]);
connection.execute(
    "INSERT INTO test (data) VALUES (?1)",
    params![values],
).unwrap();
```
