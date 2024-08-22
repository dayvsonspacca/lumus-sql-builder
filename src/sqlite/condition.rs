use super::{escape_value, BuildableStatement};
use crate::errors::SqlBuilderError;

/// Represents a WHERE clause builder for SQL queries.
#[derive(Debug)]
pub struct Where {
    statement: String,
}

impl Where {
    /// Creates a new `Where` instance with an empty statement.
    /// # Example
    /// ```
    /// use lumus_sql_builder::sqlite::Where;
    ///
    /// let mut condition = Where::new();
    /// condition.equal_to("name", "Dayvson Spacca");
    ///
    /// assert_eq!(condition.build(), "name = 'Dayvson Spacca'")
    /// ```
    pub fn new() -> Self {
        Self {
            statement: String::new(),
        }
    }

    /// Creates a new `Where` instance with a specified initial statement.
    /// # Example
    /// ```
    /// use lumus_sql_builder::sqlite::Where;
    ///
    /// let mut condition = Where::from("name = 'Dayvson Spacca'");
    /// condition.and().greater_than("age", "21");
    ///
    /// assert_eq!(condition.build(), "name = 'Dayvson Spacca' AND age > '21'");
    /// ```
    pub fn from(statement: &str) -> Self {
        Self {
            statement: statement.to_string(),
        }
    }

    /// Adds an equality condition (`field = value`) to the WHERE clause.
    pub fn equal_to(&mut self, field: &str, value: &str) -> &mut Self {
        self.add_predicate(field, "=", escape_value(value).as_str())
            .unwrap();
        self
    }

    /// Adds a not equal condition (`field != value`) to the WHERE clause.
    pub fn not_equal_to(&mut self, field: &str, value: &str) -> &mut Self {
        self.add_predicate(field, "!=", escape_value(value).as_str())
            .unwrap();
        self
    }

    /// Adds a greater than condition (`field > value`) to the WHERE clause.
    pub fn greater_than(&mut self, field: &str, value: &str) -> &mut Self {
        self.add_predicate(field, ">", escape_value(value).as_str())
            .unwrap();
        self
    }

    /// Adds a greater than or equal condition (`field >= value`) to the WHERE clause.
    pub fn greater_than_equal(&mut self, field: &str, value: &str) -> &mut Self {
        self.add_predicate(field, ">=", escape_value(value).as_str())
            .unwrap();
        self
    }

    /// Adds a less than condition (`field < value`) to the WHERE clause.
    pub fn less_than(&mut self, field: &str, value: &str) -> &mut Self {
        self.add_predicate(field, "<", escape_value(value).as_str())
            .unwrap();
        self
    }

    /// Adds a less than or equal condition (`field <= value`) to the WHERE clause.
    pub fn less_than_equal(&mut self, field: &str, value: &str) -> &mut Self {
        self.add_predicate(field, "<=", escape_value(value).as_str())
            .unwrap();
        self
    }

    /// Adds a `IS NULL` condition (`field IS NULL`) to the WHERE clause.
    pub fn is_null(&mut self, field: &str) -> &mut Self {
        self.add_predicate(field, "IS NULL", "").unwrap();
        self
    }

    /// Adds a `IS NOT NULL` condition (`field IS NOT NULL`) to the WHERE clause.
    pub fn is_not_null(&mut self, field: &str) -> &mut Self {
        self.add_predicate(field, "IS NOT NULL", "").unwrap();
        self
    }

    /// Adds an `IN` condition (`field IN (values)`) to the WHERE clause.
    pub fn inside(&mut self, field: &str, values: Vec<&str>) -> &mut Self {
        self.add_predicate(
            field,
            "IN",
            &format!(
                "({})",
                values
                    .iter()
                    .map(|v| escape_value(v))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        )
        .unwrap();
        self
    }

    /// Adds a `NOT IN` condition (`field NOT IN (values)`) to the WHERE clause.
    pub fn not_inside(&mut self, field: &str, values: Vec<&str>) -> &mut Self {
        self.add_predicate(
            field,
            "NOT IN",
            &format!(
                "({})",
                values
                    .iter()
                    .map(|v| escape_value(v))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        )
        .unwrap();
        self
    }

    /// Adds a `LIKE` condition (`field LIKE value`) to the WHERE clause.
    pub fn like(&mut self, field: &str, value: &str) -> &mut Self {
        self.add_predicate(field, "LIKE", escape_value(value).as_str())
            .unwrap();
        self
    }

    /// Adds a `NOT LIKE` condition (`field NOT LIKE value`) to the WHERE clause.
    pub fn not_like(&mut self, field: &str, value: &str) -> &mut Self {
        self.add_predicate(field, "NOT LIKE", escape_value(value).as_str())
            .unwrap();
        self
    }

    /// Appends `AND` to the current statement in the WHERE clause.
    pub fn and(&mut self) -> &mut Self {
        self.statement.push_str(" AND ");
        self
    }

    /// Appends `OR` to the current statement in the WHERE clause.
    pub fn or(&mut self) -> &mut Self {
        self.statement.push_str(" OR ");
        self
    }

    /// Appends a left parenthesis `(` to the current statement in the WHERE clause.
    pub fn nest(&mut self) -> &mut Self {
        self.statement.push('(');
        self
    }

    /// Appends a right parenthesis `)` to the current statement in the WHERE clause.
    pub fn unnest(&mut self) -> &mut Self {
        self.statement.push(')');
        self
    }

    /// Constructs and returns the final SQL statement represented by the WHERE clause.
    pub fn build(&self) -> String {
        self.statement.trim().to_string()
    }

    /// Internal method to add a predicate (`field predicate value`) to the WHERE clause.
    fn add_predicate(
        &mut self,
        field: &str,
        predicate: &str,
        value: &str,
    ) -> Result<&mut Self, SqlBuilderError> {
        if field.is_empty() {
            return Err(SqlBuilderError::EmptyColumnName);
        }

        if predicate == "IS NULL" || predicate == "IS NOT NULL" {
            self.statement.push_str(&format!("{} {}", field, predicate));
            return Ok(self);
        }

        if value.is_empty() {
            return Err(SqlBuilderError::EmptyValue);
        }

        self.statement
            .push_str(&format!("{} {} {}", field, predicate, value));
        Ok(self)
    }
}

/// Implementation of the `BuildableStatement` trait for `ColumnOption`, allowing it to be printed.
impl BuildableStatement for Where {
    fn build(&self) -> String {
        self.build()
    }
}
