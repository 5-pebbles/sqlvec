use std::{str::FromStr, string::ToString};

use serde::{Deserialize, Serialize};

use rusqlite::{
    self,
    types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, Value, ValueRef},
};

/// A generic container for vectors whose contents implement ToString & FromStr.
///
/// `SqlVec` implements ToSql & FromSql storing values as `\u{F1}` delimited text, allowing for SQL operations.
/// 
/// # Example
/// ```
///  use sqlvec::SqlVec;
///  use rusqlite::{Error, Connection, params};
///
///  let conn = Connection::open_in_memory().unwrap();
///
///  // Create a table with a column that uses our custom type.
///  conn.execute(
///      "CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY, data TEXT);",
///      [],
///  ).unwrap();
///
///  // Insert a SqlVec into the table.
///  let values = SqlVec::new(vec!["one".to_string(), "two".to_string()]);
///  conn.execute(
///      "INSERT INTO test (data) VALUES (?1)",
///      params![values],
///  ).unwrap();
///
///  // Retrieve the SqlVec from the table.
///  let mut stmt = conn.prepare("SELECT data FROM test WHERE id = ?1").unwrap();
///  let mut rows = stmt.query(params![1]).unwrap();
///  let row = rows.next().unwrap().unwrap();
///  let db_values: SqlVec<String> = row.get(0).unwrap();
///
///  // Assert that the retrieved SqlVec matches the original.
///  assert_eq!(values, db_values);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
pub struct SqlVec<T: ToString + FromStr>(Vec<T>);

impl<T: ToString + FromStr> SqlVec<T> {
    /// Creates a new `SqlVec` from an iterable collection of items.
    ///
    /// # Example
    ///
    /// ```
    /// use sqlvec::SqlVec;
    ///
    /// let vec = SqlVec::new([1, 2, 3]);
    ///
    /// ```
    pub fn new<I: IntoIterator<Item = T>>(items: I) -> Self {
        let items: Vec<T> = items.into_iter().collect();
        Self(items)
    }

    /// Consumes the `SqlVec`, returning its internal vector.
    ///
    /// This method allows you to take ownership of the underlying vector contained within the `SqlVec`. After calling `into_inner`, the `SqlVec` cannot be used anymore unless recreated.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqlvec::SqlVec; 
    /// 
    /// let sql_vec = SqlVec::new(vec![1, 2]); 
    /// let vec = sql_vec.into_inner(); 
    /// 
    /// assert_eq!(vec, vec![1, 2]); 
    ///
    /// ```
    pub fn into_inner(self) -> Vec<T> {
        self.0
    }
}

impl<T: ToString + FromStr> ToString for SqlVec<T> {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("\u{F1}")
    }
}

impl FromStr for SqlVec<String> {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items: Vec<String> = s
            .split('\u{F1}')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        Ok(Self(items))
    }
}

impl<T: ToString + FromStr> ToSql for SqlVec<T> {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let items_str = self
            .0
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("\u{F1}");
        Ok(ToSqlOutput::Owned(Value::Text(items_str)))
    }
}

impl<T: ToString + FromStr> FromSql for SqlVec<T> {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let items = value
            .as_str()?
            .split('\u{F1}')
            .filter_map(|s| s.parse().ok())
            .collect();
        Ok(SqlVec(items))
    }
}
