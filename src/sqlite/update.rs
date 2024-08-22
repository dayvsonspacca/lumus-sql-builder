use super::BuildableStatement;
use crate::errors::SqlBuilderError;

/// Represents a ´UPDATE´ clause builder for SQL queries
#[derive(Debug)]
pub struct Update {
    table: String,
    pub set: Vec<(String, String)>,
    condition: Option<String>,
}

impl Update {
    /// Creates a new `Update` instance with the given table name.
    /// # Example
    /// ```
    /// use lumus_sql_builder::sqlite::{Update, Where};
    ///
    /// let mut condition = Where::new();
    /// condition.equal_to("age", "21");
    ///
    /// let update = Update::new("users_tb").set(vec![
    ///     ("name", "João")
    /// ]).condition(condition.build())
    /// .build();
    ///
    /// assert_eq!("UPDATE users_tb SET name = 'João' WHERE age = '21';", update.unwrap());
    /// ```
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            set: Vec::new(),
            condition: None,
        }
    }

    /// Sets the values to be updated.
    pub fn set<T: ToString>(mut self, set: Vec<(&str, T)>) -> Self {
        self.set = set
            .into_iter()
            .map(|(col, val)| (col.to_string(), val.to_string()))
            .collect();
        self
    }

    /// Specifies where for `Update`.
    pub fn condition(&mut self, condition: String) -> &mut Self {
        self.condition = Some(condition);
        self
    }

    pub fn build(&self) -> Result<String, SqlBuilderError> {
        if self.table.is_empty() {
            return Err(SqlBuilderError::EmptyTableName);
        }

        if self.set.is_empty() {
            return Err(SqlBuilderError::EmptyColumnAndValue);
        }

        let mut sets: Vec<String> = vec![];

        for (col, val) in &self.set {
            if col.is_empty() {
                return Err(SqlBuilderError::EmptyColumnName);
            }
            if val.is_empty() {
                return Err(SqlBuilderError::EmptyValue);
            }

            sets.push(format!("{} = '{}'", col.clone(), val.clone()));
        }

        if let Some(condition) = &self.condition {
            return Ok(format!(
                "UPDATE {} SET {} WHERE {};",
                self.table,
                sets.join(", "),
                condition
            ));
        }

        Ok(format!("UPDATE {} SET {};", self.table, sets.join(", "),))
    }
}

/// Implementation of the `BuildableStatement` trait for `Update`, allowing it to be printed.
impl BuildableStatement for Update {
    fn build(&self) -> String {
        self.build().unwrap()
    }
}
