use core::fmt;

/// Represents the creation of a table with specified columns and options.
pub struct CreateTable<'a> {
    table: &'a str,
    columns: Vec<Column>,
    if_not_exists: bool,
}

impl<'a> CreateTable<'a> {
    /// Creates a new `CreateTable` instance with the given table name and columns.
    pub fn new(table: &'a str, columns: Vec<Column>) -> CreateTable<'a> {
        CreateTable {
            table,
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
    pub fn build(&self) -> String {
        let mut statement = String::new();
        if self.if_not_exists {
            statement.push_str(&format!("CREATE TABLE IF NOT EXISTS {} ", self.table));
        } else {
            statement.push_str(&format!("CREATE TABLE {} ", self.table));
        }
        statement.push('(');
        for (i, column) in self.columns.iter().enumerate() {
            statement.push_str(&column.build());

            if i < self.columns.len() - 1 {
                statement.push_str(", ");
            }
        }

        statement.push_str(");");
        statement
    }
}

/// Implementation of the Display trait for `CreateTable`, allowing it to be printed.
impl<'a> fmt::Display for CreateTable<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
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
    literal: Option<String>,
}

impl Column {
    /// Creates a new `Column` instance with the given column name.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            column_type: None,
            options: Vec::new(),
            literal: None,
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

    /// Specifies a `literal` value for the column.
    pub fn literal(mut self, value: &str) -> Self {
        self.literal = Some(value.to_string());
        self
    }

    /// Builds and returns the SQL representation of the column.
    pub fn build(&self) -> String {
        let column_type_str = match &self.column_type {
            Some(ct) => ct.to_string(),
            None => String::new(),
        };

        let options_str: String = self
            .options
            .iter()
            .map(|opt| opt.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        if options_str.len() > 0 {
            return format!(
                "{}{}",
                self.name,
                format!(" {} {}", column_type_str, options_str)
            );
        }

        if !column_type_str.is_empty() {
            return format!("{} {}", self.name, column_type_str);
        }

        let literal_str = match &self.literal {
            Some(lit) => lit.clone(),
            None => String::new(),
        };

        return format!("{} {}", self.name, literal_str);
    }
}
