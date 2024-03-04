pub struct CreateTable<'a> {
    table: &'a str,
    columns: Vec<Column>,
    if_not_exists: bool,
}

impl<'a> CreateTable<'a> {
    pub fn new(table: &'a str, if_not_exists: bool, columns: Vec<Column>) -> CreateTable<'a> {
        CreateTable {
            table,
            columns,
            if_not_exists,
        }
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
            if i > 0 {
                statement.push_str(&format!(", {}", column.statement));
            } else {
                statement.push_str(column.statement.as_str());
            }
        }
        statement.push(')');
        statement.push(';');
        statement
    }
}

pub struct Column {
    statement: String,
}

impl Column {
    pub fn new(name: &str) -> Column {
        Column {
            statement: String::from(name),
        }
    }

    pub fn literal(mut self, expression: &str) -> Self {
        self.statement.push_str(&format!(" {}", expression));
        self
    }
}
