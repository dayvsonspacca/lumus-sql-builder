use super::BuildableStatement;
use crate::errors::SqlBuilderError;

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

/// Implementation of the `BuildableStatement` trait for `ColumnType`, allowing it to be printed.
impl BuildableStatement for ColumnType {
    fn build(&self) -> String {
        String::from(match self {
            Self::Integer => "INTEGER",
            Self::Text => "TEXT",
            Self::Real => "REAL",
            Self::Boolean => "BOOLEAN",
            Self::Blob => "BLOB",
            Self::Numeric => "NUMERIC",
            Self::Date => "DATE",
            Self::Time => "TIME",
            Self::Datetime => "DATETIME",
        })
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

/// Implementation of the `BuildableStatement` trait for `ColumnOption`, allowing it to be printed.
impl BuildableStatement for ColumnOption {
    fn build(&self) -> String {
        match self {
            Self::NotNull => "NOT NULL".to_string(),
            Self::Unique => "UNIQUE".to_string(),
            Self::Default(s) => format!("DEFAULT {}", s),
            Self::AutoIncrement => "AUTOINCREMENT".to_string(),
            Self::PrimaryKey => "PRIMARY KEY".to_string(),
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

        let column_type = match &self.column_type {
            Some(ct) => ct.build(),
            None => return Err(SqlBuilderError::InvalidColumnType),
        };

        let options_str = self
            .options
            .iter()
            .map(|opt| opt.build())
            .collect::<Vec<String>>()
            .join(" ");

        Ok(format!("{} {} {}", self.name, column_type, options_str))
    }
}

/// Implementation of the `BuildableStatement` trait for `Column`, allowing it to be printed.
impl BuildableStatement for Column {
    fn build(&self) -> String {
        self.build().unwrap()
    }
}
