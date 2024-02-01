fn main() {
    let mut select = sql_builder::Select::new();

    select.from("users_tb");
    select.columns(["name", "age", "email"].to_vec());
    select.limit(5);

    println!("{}", select.build());
}

pub mod sql_builder {

    pub struct Select {
        distinct: bool,
        from: String,
        limit: u32,
        columns: Vec<String>,
    }

    impl Select {
        pub fn new() -> Select {
            Select {
                distinct: false,
                from: String::new(),
                limit: 0,
                columns: Vec::new(),
            }
        }

        pub fn distinct(&mut self) {
            self.distinct = true;
        }

        pub fn from(&mut self, from: &str) {
            self.from = from.to_string();
        }

        pub fn columns(&mut self, columns: Vec<&str>) {
            self.columns = columns.iter().map(|s| s.to_string()).collect();
        }

        pub fn limit(&mut self, limit: u32) {
            self.limit = limit;
        }

        pub fn build(&self) -> String {
            let mut statement = "SELECT ".to_string();

            if self.distinct {
                statement.push_str("DISTINCT ");
            }

            if self.columns.len() > 0 {
                for (index, col) in self.columns.iter().enumerate() {
                    if index == self.columns.len() - 1 {
                        statement.push_str(&format!("{} ", &col));
                        continue;
                    }
                    statement.push_str(&format!("{}, ", &col));
                }
            } else {
                statement.push_str("* ");
            }

            if self.from.len() > 0 {
                statement.push_str(&format!("FROM {} ", self.from));
            }

            if self.limit > 0 {
                statement.push_str(&format!("LIMIT {};", self.limit));
            }

            statement
        }
    }
}
