pub struct Select {
    distinct: bool,
    from: String,
    limit: u32,
    columns: Vec<String>,
    group: Vec<String>,
    order: Vec<String>,
}

#[allow(dead_code)]
impl Select {
    pub fn new() -> Select {
        Select {
            distinct: false,
            from: String::new(),
            limit: 0,
            columns: Vec::new(),
            group: Vec::new(),
            order: Vec::new(),
        }
    }

    pub fn distinct(&mut self) -> &mut Select {
        self.distinct = true;
        self
    }

    pub fn from(&mut self, from: &str) -> &mut Select {
        self.from = from.to_string();
        self
    }

    pub fn columns(&mut self, columns: Vec<&str>) -> &mut Select {
        self.columns = columns.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn group(&mut self, group: Vec<&str>) -> &mut Select {
        self.group = group.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn order(&mut self, order: Vec<&str>) -> &mut Select {
        self.order = order.iter().map(|s| s.to_string()).collect();
        self
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

        if self.group.len() > 0 {
            statement.push_str("GROUP BY ");
            for (index, col) in self.group.iter().enumerate() {
                if index == self.group.len() - 1 {
                    statement.push_str(&format!("{} ", &col));
                    continue;
                }
                statement.push_str(&format!("{}, ", &col));
            }
        }

        if self.order.len() > 0 {
            statement.push_str("ORDER BY ");
            for (index, col) in self.order.iter().enumerate() {
                if index == self.order.len() - 1 {
                    statement.push_str(&format!("{} ", &col));
                    continue;
                }
                statement.push_str(&format!("{}, ", &col));
            }
        }

        if self.limit > 0 {
            statement.push_str(&format!("LIMIT {}", self.limit));
        }

        statement = statement.trim().to_string() + ";";
        statement
    }
}
