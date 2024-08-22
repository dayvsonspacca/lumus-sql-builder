use super::BuildableStatement;
use crate::errors::SqlBuilderError;

/// Represents a ´DELETE´ clause builder for SQL queries
#[derive(Debug)]
pub struct Delete {
    table: String,
    condition: Option<String>,
}

impl Delete {
    /// Creates a new `Delete` instance with the given table name.
    /// # Example
    /// ```
    /// use lumus_sql_builder::sqlite::Delete;
    ///
    /// let delete = Delete::new("users_tb").build();
    ///
    /// assert_eq!("DELETE FROM users_tb;", delete.unwrap());
    /// ```
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            condition: None,
        }
    }

    /// Specifies where for `Delete`.
    pub fn condition(&mut self, condition: String) -> &mut Self {
        self.condition = Some(condition);
        self
    }

    pub fn build(&self) -> Result<String, SqlBuilderError> {
        if self.table.is_empty() {
            return Err(SqlBuilderError::EmptyTableName);
        }

        if let Some(condition) = &self.condition {
            if condition.is_empty() {
                return Err(SqlBuilderError::EmptyCondition);
            }
            return Ok(format!("DELETE FROM {} WHERE {};", self.table, condition));
        }

        Ok(format!("DELETE FROM {};", self.table))
    }
}

/// Implementation of the `BuildableStatement` trait for `Delete`, allowing it to be printed.
impl BuildableStatement for Delete {
    fn build(&self) -> String {
        self.build().unwrap()
    }
}
