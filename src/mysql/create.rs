pub struct Create<'a> {
    columns: &'a str,
    table: &'a str,
}

#[allow(dead_code)]
impl<'a> Create<'a> {
    pub fn new(table: &'a str) -> Create<'a> {
        Create { table, columns: "" }
    }

    pub fn columns(&mut self, columns: &'a str) -> &Create<'a> {
        self.columns = columns;
        self
    }

    pub fn build(&self) -> String {
        let mut statement = "CREATE TABLE IF NOT EXISTS ".to_string();
        statement.push_str(&format!("{}", self.table));
        if self.columns.len() > 0 {
            statement.push_str(&format!(" ({})", self.columns));
        }

        statement = statement.trim().to_string() + ";";
        statement
    }
}
