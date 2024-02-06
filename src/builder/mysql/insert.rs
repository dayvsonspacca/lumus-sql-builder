pub struct Insert {
    table: String,
    values: Vec<[String; 2]>,
}

impl Insert {
    pub fn new(table: &str) -> Insert {
        Insert {
            table: table.to_string(),
            values: vec![],
        }
    }

    pub fn values(&mut self, values: Vec<[String; 2]>) {
        self.values = values;
    }

    pub fn build(&self) -> String {
        let mut statement = "INSERT INTO".to_string();

        statement.push_str(&format!(" {} ", self.table));

        let mut columns = String::new();
        let mut values = String::new();

        for column in &self.values {
            columns.push_str(&format!("{},", column[0]));

            if column[1].parse::<f64>().is_ok() {
                values.push_str(&format!("{},", column[1]));
            } else {
                values.push_str(&format!("'{}',", column[1]));
            }
        }
        columns.pop();
        values.pop();
        
        statement.push_str(&format!("({}) VALUES ({})", columns, values));

        statement
    }
}
