use super::BuildableStatement;
use crate::errors::SqlBuilderError;

/// Represents the possible join types.
#[derive(Debug)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    RightOuter,
    LeftOuter,
    Full,
}

/// Implementation of the `BuildableStatement` trait for `JoinType`, allowing it to be printed.
impl BuildableStatement for JoinType {
    fn build(&self) -> String {
        String::from(match self {
            Self::Inner => "INNER",
            Self::Left => "LEFT",
            Self::Right => "RIGHT",
            Self::RightOuter => "RIGHT OUTER",
            Self::LeftOuter => "LEFT OUTER",
            Self::Full => "FULL",
        })
    }
}

/// Represents a ´JOIN´ clause builder for SQL queries
#[derive(Debug)]
pub struct Join {
    table: String,
    join_type: JoinType,
    on: String,
}

impl Join {
    /// Creates a new `Select` instance with the specified table name.
    /// # Example
    /// ```
    /// use lumus_sql_builder::sqlite::{Join, JoinType};
    /// let join = Join::new("phones p", JoinType::Inner, "p.user_id = u.user_id").build().unwrap();
    /// assert_eq!(join, "INNER JOIN phones p ON p.user_id = u.user_id");
    /// ```
    pub fn new(table: &str, join_type: JoinType, on: &str) -> Self {
        Self {
            table: table.to_string(),
            join_type,
            on: on.to_string(),
        }
    }

    pub fn build(self) -> Result<String, SqlBuilderError> {
        if self.table.is_empty() {
            return Err(SqlBuilderError::EmptyTableName);
        }

        if self.on.is_empty() {
            return Err(SqlBuilderError::EmptyOnClause);
        }

        Ok(format!(
            "{} JOIN {} ON {}",
            self.join_type.build(),
            self.table,
            self.on
        ))
    }
}
