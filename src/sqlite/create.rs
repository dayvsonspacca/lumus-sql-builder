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
            statement.push_str(&column.statement);

            if i < self.columns.len() - 1 {
                statement.push_str(", ");
            }
        }

        statement.push_str(");");
        statement
    }
}
pub struct Column {
    statement: String,
}

impl Column {
    pub fn new(name: &str) -> Column {
        Column {
            statement: format!("{}", name),
        }
    }

    pub fn literal(mut self, expression: &str) -> Self {
        self.statement.push_str(&format!(" {}", expression));
        self
    }
    
    pub fn integer(mut self) -> Self {
        self.statement.push_str(" INTEGER");
        self
    }
    
    pub fn text(mut self) -> Self {
        self.statement.push_str(" TEXT");
        self
    }
    
    pub fn real(mut self) -> Self {
        self.statement.push_str(" REAL");
        self
    }
    
    pub fn boolean(mut self) -> Self {
        self.statement.push_str(" BOOLEAN");
        self
    }
    
    pub fn blob(mut self) -> Self {
        self.statement.push_str(" BLOB");
        self
    }
    
    pub fn numeric(mut self) -> Self {
        self.statement.push_str(" NUMERIC");
        self
    }
    
    pub fn date(mut self) -> Self {
        self.statement.push_str(" DATE");
        self
    }
    
    pub fn time(mut self) -> Self {
        self.statement.push_str(" TIME");
        self
    }
    
    pub fn datetime(mut self) -> Self {
        self.statement.push_str(" DATETIME");
        self
    }
    
    pub fn check(mut self, condition: &str) -> Self {
        self.statement.push_str(&format!(" CHECK ({})", condition));
        self
    }        

    pub fn not_null(mut self) -> Self {
        self.statement.push_str(" NOT NULL");
        self
    }

    pub fn unique(mut self) -> Self {
        self.statement.push_str(" UNIQUE");
        self
    }

    pub fn default(mut self, value: &str) -> Self {
        self.statement.push_str(&format!(" DEFAULT {}", value));
        self
    }

    pub fn auto_increment(mut self) -> Self {
        self.statement.push_str(" AUTOINCREMENT");
        self
    }

    pub fn primary_key(mut self) -> Self {
        self.statement.push_str(" PRIMARY KEY");
        self
    }

    pub fn build(self) -> String {
        self.statement
    }
}
 