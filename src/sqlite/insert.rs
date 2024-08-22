use super::BuildableStatement;
use crate::errors::SqlBuilderError;

/// Represents the creation of a INSERT with specified table and values.
#[derive(Debug)]
pub struct Insert {
    pub table: String,
    pub values: Vec<(String, String)>,
}

impl Insert {
    /// Creates a new `Insert` instance with the given table name.
    /// # Example
    /// ```
    /// use lumus_sql_builder::sqlite::Insert;
    ///
    /// let insert = Insert::new("metas_clientes_tb").values(vec![
    ///     ("name", "João"),
    ///     ("age", "30"),
    ///     ("department", "TI"),
    ///     ("salary", "5000.00"),
    ///     ("hired_date", "2024-03-20"),
    ///     ("manager_id", "1"),
    /// ]).build().unwrap();
    /// ```
    /// assert_eq!(insert, "INSERT INTO metas_clientes_tb (name, age, department, salary, hired_date, manager_id) VALUES ('João', '30', 'TI', '5000.00', '2024-03-20', '1');")
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            values: Vec::new(),
        }
    }

    /// Sets the values to be inserted.
    pub fn values(mut self, values: Vec<(&str, &str)>) -> Self {
        self.values = values
            .into_iter()
            .map(|(col, val)| (col.to_string(), val.to_string()))
            .collect();
        self
    }

    /// Builds and returns the SQL statement for the `INSERT` query.
    pub fn build(&self) -> Result<String, SqlBuilderError> {
        if self.table.is_empty() {
            return Err(SqlBuilderError::EmptyTableName);
        }

        if self.values.is_empty() {
            return Err(SqlBuilderError::EmptyColumnAndValue);
        }

        let mut columns: Vec<String> = vec![];
        let mut values: Vec<String> = vec![];

        for (col, val) in &self.values {
            if col.is_empty() {
                return Err(SqlBuilderError::EmptyColumnName);
            }
            if val.is_empty() {
                return Err(SqlBuilderError::EmptyValue);
            }

            columns.push(col.clone());
            values.push(format!("'{}'", val.clone()));
        }

        Ok(format!(
            "INSERT INTO {} ({}) VALUES ({});",
            self.table,
            columns.join(", "),
            values.join(", ")
        ))
    }
}

/// Implementation of the `BuildableStatement` trait for `Delete`, allowing it to be printed.
impl BuildableStatement for Insert {
    fn build(&self) -> String {
        self.build().unwrap()
    }
}
