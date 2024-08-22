use super::{BuildableStatement, Column};
use crate::errors::SqlBuilderError;

/// Represents the creation of a table with specified columns and options.
#[derive(Debug)]
pub struct CreateTable {
    table: String,
    columns: Vec<Column>,
    if_not_exists: bool,
}

impl CreateTable {
    /// Creates a new `CreateTable` instance with the given table name and columns.
    /// # Example
    /// ```
    /// use lumus_sql_builder::sqlite::{CreateTable, Column};
    /// let create_table = CreateTable::new("users", vec![
    ///     Column::new("name").text().not_null().primary_key(),
    /// ]).build().unwrap();
    ///
    /// assert_eq!(create_table, "CREATE TABLE users (name TEXT NOT NULL PRIMARY KEY);");
    /// ```
    pub fn new(table: &str, columns: Vec<Column>) -> Self {
        Self {
            table: table.to_string(),
            columns,
            if_not_exists: false,
        }
    }

    /// Specifies that the table should be created only if it does not already exist.
    pub fn if_not_exists(mut self) -> Self {
        self.if_not_exists = true;
        self
    }

    /// Builds and returns the SQL statement for creating the table.
    pub fn build(&self) -> Result<String, SqlBuilderError> {
        if self.table.is_empty() {
            return Err(SqlBuilderError::EmptyTableName);
        }

        if self.columns.is_empty() {
            return Err(SqlBuilderError::NoColumnsSpecified);
        }

        let mut statement = if self.if_not_exists {
            format!("CREATE TABLE IF NOT EXISTS {} (", self.table)
        } else {
            format!("CREATE TABLE {} (", self.table)
        };

        let columns_sql: Result<Vec<String>, SqlBuilderError> =
            self.columns.iter().map(|col| col.build()).collect();

        statement.push_str(&columns_sql?.join(", "));
        statement.push_str(");");

        Ok(statement)
    }
}

/// Implementation of the `BuildableStatement` trait for `CreateTable`, allowing it to be printed.
impl BuildableStatement for CreateTable {
    fn build(&self) -> String {
        self.build().unwrap()
    }
}
