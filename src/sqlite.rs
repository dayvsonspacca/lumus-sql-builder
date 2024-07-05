use crate::errors::SqlBuilderError;
use core::fmt;

/// Represents the creation of a table with specified columns and options.
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
    pub fn new<T: Into<String>>(table: T, columns: Vec<Column>) -> CreateTable {
        CreateTable {
            table: table.into(),
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
    /// Column::new("name").text().not_null();
    /// ```
    pub fn new(name: &str) -> Column {
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
    pub fn new<T: Into<String>>(table: T) -> Select {
        Select {
            table: table.into(),
            distinct: false,
            columns: None,
            condition: None,
            group: None,
            order: None,
            limit: None,
            offset: None,
        }
    }

    /// Specifies that the select statement should return distinct rows.
    pub fn distinct(&mut self) -> &mut Self {
        self.distinct = true;
        self
    }

    /// Specifies the columns to be selected in the query.
    pub fn columns<T: Into<String>>(&mut self, columns: T) -> &mut Self {
        self.columns = Some(columns.into());
        self
    }

    /// Specifies the grouping for the query results.
    pub fn group<T: Into<String>>(&mut self, group: T) -> &mut Self {
        self.group = Some(group.into());
        self
    }

    /// Specifies the ordering for the query results.
    pub fn order<T: Into<String>>(&mut self, order: T) -> &mut Self {
        self.order = Some(order.into());
        self
    }

    /// Specifies where for `Update`, `Delete` and `Select`.
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

impl fmt::Display for Select {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.build() {
            Err(e) => write!(f, "{}", e),
            Ok(s) => write!(f, "{}", s),
        }
    }
}

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
    pub fn new<T: Into<String>>(table: T) -> Insert {
        Insert {
            table: table.into(),
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
        self.add_predicate(field, "=", value).unwrap();
        self
    }

    /// Adds a not equal condition (`field != value`) to the WHERE clause.
    pub fn not_equal_to(&mut self, field: &str, value: &str) -> &mut Self {
        self.add_predicate(field, "!=", value).unwrap();
        self
    }

    /// Adds a greater than condition (`field > value`) to the WHERE clause.
    pub fn greater_than(&mut self, field: &str, value: &str) -> &mut Self {
        self.add_predicate(field, ">", value).unwrap();
        self
    }

    /// Adds a greater than or equal condition (`field >= value`) to the WHERE clause.
    pub fn greater_than_equal(&mut self, field: &str, value: &str) -> &mut Self {
        self.add_predicate(field, ">=", value).unwrap();
        self
    }

    /// Adds a less than condition (`field < value`) to the WHERE clause.
    pub fn less_than(&mut self, field: &str, value: &str) -> &mut Self {
        self.add_predicate(field, "<", value).unwrap();
        self
    }

    /// Adds a less than or equal condition (`field <= value`) to the WHERE clause.
    pub fn less_than_equal(
        &mut self,
        field: &str,
        value: &str,
    ) -> Result<&mut Self, SqlBuilderError> {
        self.add_predicate(field, "<=", value)
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
        self.add_predicate(field, "IN", &format!("({})", values.join(", ")))
            .unwrap();
        self
    }

    /// Adds a `NOT IN` condition (`field NOT IN (values)`) to the WHERE clause.
    pub fn not_inside(&mut self, field: &str, values: Vec<&str>) -> &mut Self {
        self.add_predicate(field, "NOT IN", &format!("({})", values.join(", ")))
            .unwrap();
        self
    }

    /// Adds a `LIKE` condition (`field LIKE value`) to the WHERE clause.
    pub fn like(&mut self, field: &str, value: &str) -> &mut Self {
        self.add_predicate(field, "LIKE", value).unwrap();
        self
    }

    /// Adds a `NOT LIKE` condition (`field NOT LIKE value`) to the WHERE clause.
    pub fn not_like(&mut self, field: &str, value: &str) -> &mut Self {
        self.add_predicate(field, "NOT LIKE", value).unwrap();
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
            self.statement
                .push_str(&format!("{} {} ", field, predicate));
            return Ok(self);
        }

        if value.is_empty() {
            return Err(SqlBuilderError::EmptyValue);
        }

        let escaped_value = format!("'{}'", value);

        self.statement
            .push_str(&format!("{} {} {}", field, predicate, escaped_value));
        Ok(self)
    }
}

/// Implementation of the Display trait for `Where`, allowing it to be printed.
impl fmt::Display for Where {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

pub struct Update {
    table: String,
    pub set: Vec<(String, String)>,
    condition: Option<String>,
}

impl Update {
    pub fn new<T: Into<String>>(table: T) -> Self {
        Update {
            table: table.try_into().unwrap(),
            set: Vec::new(),
            condition: None,
        }
    }

    pub fn build(self) -> Result<String, SqlBuilderError> {
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

        if let Some(condition) = self.condition {
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
