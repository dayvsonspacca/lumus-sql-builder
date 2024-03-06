use core::fmt;

pub struct CreateTable<'a> {
    table: &'a str,
    columns: Vec<Column>,
    if_not_exists: bool,
}

impl<'a> CreateTable<'a> {
    pub fn new(table: &'a str, columns: Vec<Column>) -> CreateTable<'a> {
        CreateTable {
            table,
            columns,
            if_not_exists: false,
        }
    }

    pub fn if_not_exists(mut self) -> Self {
        self.if_not_exists = true;
        self
    }

    pub fn build(self) -> String {
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

#[derive(Debug)]
pub enum ColumnOption {
    NotNull,
    Unique,
    Default(String),
    AutoIncrement,
    PrimaryKey,
}

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

#[derive(Debug)]
pub struct Column {
    name: String,
    column_type: Option<ColumnType>,
    options: Vec<ColumnOption>,
}

impl Column {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            column_type: None,
            options: Vec::new(),
        }
    }

    pub fn integer(mut self) -> Self {
        self.column_type = Some(ColumnType::Integer);
        self
    }

    pub fn text(mut self) -> Self {
        self.column_type = Some(ColumnType::Text);
        self
    }

    pub fn real(mut self) -> Self {
        self.column_type = Some(ColumnType::Real);
        self
    }

    pub fn boolean(mut self) -> Self {
        self.column_type = Some(ColumnType::Boolean);
        self
    }

    pub fn blob(mut self) -> Self {
        self.column_type = Some(ColumnType::Blob);
        self
    }

    pub fn numeric(mut self) -> Self {
        self.column_type = Some(ColumnType::Numeric);
        self
    }

    pub fn date(mut self) -> Self {
        self.column_type = Some(ColumnType::Date);
        self
    }

    pub fn time(mut self) -> Self {
        self.column_type = Some(ColumnType::Time);
        self
    }

    pub fn datetime(mut self) -> Self {
        self.column_type = Some(ColumnType::Datetime);
        self
    }

    pub fn not_null(mut self) -> Self {
        self.options.push(ColumnOption::NotNull);
        self
    }

    pub fn unique(mut self) -> Self {
        self.options.push(ColumnOption::Unique);
        self
    }

    pub fn default(mut self, value: &str) -> Self {
        self.options.push(ColumnOption::Default(value.to_string()));
        self
    }

    pub fn auto_increment(mut self) -> Self {
        self.options.push(ColumnOption::AutoIncrement);
        self
    }

    pub fn primary_key(mut self) -> Self {
        self.options.push(ColumnOption::PrimaryKey);
        self
    }

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

        format!("{} {}", self.name, column_type_str)
    }
}
