use super::BuildableStatement;
use crate::errors::SqlBuilderError;

/// Represents the creation of a SELECT with specified table and options.
#[derive(Debug)]
pub struct Select {
    table: String,
    distinct: bool,
    condition: Option<String>,
    columns: Option<String>,
    group: Option<String>,
    order: Option<String>,
    limit: Option<u32>,
    offset: Option<u32>,
}

impl Select {
    /// Creates a new `Select` instance with the specified table name.
    /// # Example
    /// ```
    /// use lumus_sql_builder::sqlite::Select;
    /// let select = Select::new("users").columns("name, age").build().unwrap();
    /// assert_eq!(select, "SELECT name, age FROM users;")
    /// ```
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            distinct: false,
            columns: None,
            condition: None,
            group: None,
            order: None,
            limit: None,
            offset: None,
        }
    }

    /// Creates a new `Select` instance from a SQL query string.
    /// The query string should be in the format "SELECT * FROM table_name [WHERE condition] [GROUP BY column] [ORDER BY column] [LIMIT limit] [OFFSET offset]".
    /// # Example
    /// ```
    /// use lumus_sql_builder::sqlite::Select;
    /// let query = "SELECT * FROM users WHERE age > 18 GROUP BY city ORDER BY name LIMIT 10 OFFSET 0";
    /// Select::from(query);
    /// ```
    pub fn from(query: &str) -> Result<Select, SqlBuilderError> {
        let mut parts = query.split_whitespace();
        let select = parts.next().ok_or(SqlBuilderError::InvalidQuery)?;
        if select.to_uppercase() != "SELECT" {
            return Err(SqlBuilderError::InvalidQuery);
        }

        let _ = parts.next().ok_or(SqlBuilderError::InvalidQuery)?; // Skip the "*"

        let from = parts.next().ok_or(SqlBuilderError::InvalidQuery)?;
        if from.to_uppercase() != "FROM" {
            return Err(SqlBuilderError::InvalidQuery);
        }

        let table = parts.next().ok_or(SqlBuilderError::InvalidQuery)?;

        let mut select_builder = Select::new(table);

        while let Some(part) = parts.next() {
            match part.to_uppercase().as_str() {
                "WHERE" => {
                    let condition = parts.next().ok_or(SqlBuilderError::InvalidQuery)?;
                    select_builder.condition(condition.to_string());
                }
                "GROUP" => {
                    let by = parts.next().ok_or(SqlBuilderError::InvalidQuery)?;
                    if by.to_uppercase() != "BY" {
                        return Err(SqlBuilderError::InvalidQuery);
                    }
                    let group = parts.next().ok_or(SqlBuilderError::InvalidQuery)?;
                    select_builder.group(group);
                }
                "ORDER" => {
                    let by = parts.next().ok_or(SqlBuilderError::InvalidQuery)?;
                    if by.to_uppercase() != "BY" {
                        return Err(SqlBuilderError::InvalidQuery);
                    }
                    let order = parts.next().ok_or(SqlBuilderError::InvalidQuery)?;
                    select_builder.order(order);
                }
                "LIMIT" => {
                    let limit = parts.next().ok_or(SqlBuilderError::InvalidQuery)?;
                    let limit = limit
                        .parse::<u32>()
                        .map_err(|_| SqlBuilderError::InvalidQuery)?;
                    select_builder.limit(limit);
                }
                "OFFSET" => {
                    let offset = parts.next().ok_or(SqlBuilderError::InvalidQuery)?;
                    let offset = offset
                        .parse::<u32>()
                        .map_err(|_| SqlBuilderError::InvalidQuery)?;
                    select_builder.offset(offset);
                }
                _ => return Err(SqlBuilderError::InvalidQuery),
            }
        }

        Ok(select_builder)
    }

    /// Specifies that the select statement should return distinct rows.
    pub fn distinct(&mut self) -> &mut Self {
        self.distinct = true;
        self
    }

    /// Specifies the columns to be selected in the query.
    pub fn columns(&mut self, columns: &str) -> &mut Self {
        self.columns = Some(columns.to_string());
        self
    }

    /// Specifies the grouping for the query results.
    pub fn group(&mut self, group: &str) -> &mut Self {
        self.group = Some(group.to_string());
        self
    }

    /// Specifies the ordering for the query results.
    pub fn order(&mut self, order: &str) -> &mut Self {
        self.order = Some(order.to_string());
        self
    }

    /// Specifies where for `Select`.
    pub fn condition(&mut self, condition: String) -> &mut Self {
        self.condition = Some(condition);
        self
    }

    /// Specifies the maximum number of rows to be returned by the query.
    pub fn limit(&mut self, limit: u32) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    /// Specifies the offset for the query results.
    pub fn offset(&mut self, offset: u32) -> &mut Self {
        self.offset = Some(offset);
        self
    }

    /// Builds and returns the SQL statement for the select query.
    pub fn build(&self) -> Result<String, SqlBuilderError> {
        if self.table.is_empty() {
            return Err(SqlBuilderError::EmptyTableName);
        }

        let mut statement = String::from("SELECT");

        if self.distinct {
            statement.push_str(" DISTINCT");
        }

        if let Some(columns) = &self.columns {
            statement.push_str(&format!(" {}", columns));
        } else {
            statement.push_str(" *");
        }

        statement.push_str(&format!(" FROM {}", self.table));

        if let Some(condition) = &self.condition {
            statement.push_str(&format!(" WHERE {}", condition));
        }

        if let Some(group) = &self.group {
            statement.push_str(&format!(" GROUP BY {}", group));
        }

        if let Some(order) = &self.order {
            statement.push_str(&format!(" ORDER BY {}", order));
        }

        if let Some(limit) = &self.limit {
            statement.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = &self.offset {
            statement.push_str(&format!(" OFFSET {}", offset));
        }

        statement.push(';');
        Ok(statement)
    }
}

/// Implementation of the `BuildableStatement` trait for `Select`, allowing it to be printed.
impl BuildableStatement for Select {
    fn build(&self) -> String {
        self.build().unwrap()
    }
}
