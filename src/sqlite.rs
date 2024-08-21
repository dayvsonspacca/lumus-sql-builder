use crate::{errors::SqlBuilderError, escape_value};
use core::fmt;

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
    /// CreateTable::new("users", vec![
    ///     Column::new("name").text().not_null().primary_key(),
    /// ]);
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

/// Implementation of the Display trait for `CreateTable`, allowing it to be printed.
impl fmt::Display for CreateTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.build() {
            Err(e) => write!(f, "{}", e),
            Ok(s) => write!(f, "{}", s),
        }
    }
}

/// Represents the possible data types for a table column.
#[derive(Debug)]
pub enum ColumnType {
    Integer,
    Text,
    Real,
    Boolean,
    Blob,
    Numeric,
    Date,
    Time,
    Datetime,
}

/// Implementation of the Display trait for `ColumnType`, allowing it to be printed.
impl fmt::Display for ColumnType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColumnType::Integer => write!(f, "INTEGER"),
            ColumnType::Text => write!(f, "TEXT"),
            ColumnType::Real => write!(f, "REAL"),
            ColumnType::Boolean => write!(f, "BOOLEAN"),
            ColumnType::Blob => write!(f, "BLOB"),
            ColumnType::Numeric => write!(f, "NUMERIC"),
            ColumnType::Date => write!(f, "DATE"),
            ColumnType::Time => write!(f, "TIME"),
            ColumnType::Datetime => write!(f, "DATETIME"),
        }
    }
}

/// Represents the possible options for a table column.
#[derive(Debug)]
pub enum ColumnOption {
    NotNull,
    Unique,
    Default(String),
    AutoIncrement,
    PrimaryKey,
}

/// Implementation of the Display trait for `ColumnOption`, allowing it to be printed.
impl fmt::Display for ColumnOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColumnOption::NotNull => write!(f, "NOT NULL"),
            ColumnOption::Unique => write!(f, "UNIQUE"),
            ColumnOption::Default(value) => write!(f, "DEFAULT {}", value),
            ColumnOption::AutoIncrement => write!(f, "AUTOINCREMENT"),
            ColumnOption::PrimaryKey => write!(f, "PRIMARY KEY"),
        }
    }
}

/// Represents a table column with a name, data type, and options.
#[derive(Debug)]
pub struct Column {
    name: String,
    column_type: Option<ColumnType>,
    options: Vec<ColumnOption>,
}

impl Column {
    /// Creates a new `Column` instance with the given column name.
    /// # Example
    /// ```
    /// use lumus_sql_builder::sqlite::Column;
    /// let col = Column::new("name").text().not_null();
    ///
    /// assert_eq!(col.build().unwrap(), "name TEXT NOT NULL");
    /// ```
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            column_type: None,
            options: Vec::new(),
        }
    }

    /// Specifies that the column has an `INTEGER` data type.
    pub fn integer(mut self) -> Self {
        self.column_type = Some(ColumnType::Integer);
        self
    }

    /// Specifies that the column has a `TEXT` data type.
    pub fn text(mut self) -> Self {
        self.column_type = Some(ColumnType::Text);
        self
    }

    /// Specifies that the column has a `REAL` data type.
    pub fn real(mut self) -> Self {
        self.column_type = Some(ColumnType::Real);
        self
    }

    /// Specifies that the column has a `BOOLEAN` data type.
    pub fn boolean(mut self) -> Self {
        self.column_type = Some(ColumnType::Boolean);
        self
    }

    /// Specifies that the column has a `BLOB` data type.
    pub fn blob(mut self) -> Self {
        self.column_type = Some(ColumnType::Blob);
        self
    }

    /// Specifies that the column has a `NUMERIC` data type.
    pub fn numeric(mut self) -> Self {
        self.column_type = Some(ColumnType::Numeric);
        self
    }

    /// Specifies that the column has a `DATE` data type.
    pub fn date(mut self) -> Self {
        self.column_type = Some(ColumnType::Date);
        self
    }

    /// Specifies that the column has a `TIME` data type.
    pub fn time(mut self) -> Self {
        self.column_type = Some(ColumnType::Time);
        self
    }

    /// Specifies that the column has a `DATETIME` data type.
    pub fn datetime(mut self) -> Self {
        self.column_type = Some(ColumnType::Datetime);
        self
    }

    /// Specifies that the column cannot have `NULL` values.
    pub fn not_null(mut self) -> Self {
        self.options.push(ColumnOption::NotNull);
        self
    }

    /// Specifies that the column values must be unique across rows.
    pub fn unique(mut self) -> Self {
        self.options.push(ColumnOption::Unique);
        self
    }

    /// Specifies a default value for the column.
    pub fn default(mut self, value: &str) -> Self {
        self.options.push(ColumnOption::Default(value.to_string()));
        self
    }

    /// Specifies that the column values should auto-increment.
    pub fn auto_increment(mut self) -> Self {
        self.options.push(ColumnOption::AutoIncrement);
        self
    }

    /// Specifies that the column is a primary key.
    pub fn primary_key(mut self) -> Self {
        self.options.push(ColumnOption::PrimaryKey);
        self
    }

    /// Builds and returns the SQL representation of the column.
    pub fn build(&self) -> Result<String, SqlBuilderError> {
        if self.name.is_empty() {
            return Err(SqlBuilderError::EmptyColumnName);
        }

        let column_type_str = match &self.column_type {
            Some(ct) => ct.to_string(),
            None => return Err(SqlBuilderError::InvalidColumnType),
        };

        let options_str = self
            .options
            .iter()
            .map(|opt| opt.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        Ok(format!("{} {} {}", self.name, column_type_str, options_str))
    }
}

/// Implementation of the Display trait for `Column`, allowing it to be printed.
impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.build() {
            Err(e) => write!(f, "{}", e),
            Ok(s) => write!(f, "{}", s),
        }
    }
}

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
    /// Select::new("users").columns("name, age");
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

/// Implementation of the Display trait for `Select`, allowing it to be printed.
impl fmt::Display for Select {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.build() {
            Err(e) => write!(f, "{}", e),
            Ok(s) => write!(f, "{}", s),
        }
    }
}

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
    /// Insert::new("metas_clientes_tb").values(vec![
    ///     ("name", "João"),
    ///     ("age", "30"),
    ///     ("department", "TI"),
    ///     ("salary", "5000.00"),
    ///     ("hired_date", "2024-03-20"),
    ///     ("manager_id", "1"),
    /// ]);
    /// ```
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            values: Vec::new(),
        }
    }

    /// Sets the values to be inserted.
    pub fn values<T: ToString>(mut self, values: Vec<(&str, T)>) -> Self {
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

/// Implementation of the Display trait for `Insert`, allowing it to be printed.
impl fmt::Display for Insert {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.build() {
            Err(e) => write!(f, "{}", e),
            Ok(s) => write!(f, "{}", s),
        }
    }
}
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

/// Implementation of the Display trait for `Where`, allowing it to be printed.
impl fmt::Display for Where {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

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

/// Implementation of the Display trait for `Update`, allowing it to be printed.
impl fmt::Display for Update {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.build() {
            Err(e) => write!(f, "{}", e),
            Ok(s) => write!(f, "{}", s),
        }
    }
}

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

/// Implementation of the Display trait for `Delete`, allowing it to be printed.
impl fmt::Display for Delete {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.build() {
            Err(e) => write!(f, "{}", e),
            Ok(s) => write!(f, "{}", s),
        }
    }
}
