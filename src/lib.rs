use std::{str::FromStr, string::ToString};

use serde::{Deserialize, Serialize};

use rusqlite::{
    self,
    types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, Value, ValueRef},
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SqlVec<T: ToString + FromStr>(Vec<T>);

impl<T: ToString + FromStr> SqlVec<T> {
    pub fn new<I: IntoIterator<Item = T>>(items: I) -> Self {
        let items: Vec<T> = items.into_iter().collect();
        Self(items)
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
