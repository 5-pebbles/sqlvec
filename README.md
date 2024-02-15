# SqlVec

A generic container for vectors allowing for rusqlite operations.

The vector must contain elements that implement `ToString` & `FromStr`.

`SqlVec` implements `ToSql` & `FromSql` storing values as `\u{F1}` delimited text.

> If the sqlite conversion is to be bidirectional then the `ToString` & `FromStr` must also be bidirectional.

```toml
[dependencies]
sqlvec = { version = "0.0.1", features = ["serde"] }
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
