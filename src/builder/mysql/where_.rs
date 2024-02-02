#[allow(dead_code)]
pub struct Where {
    statement: String,
    combined_by: Combiner,
}

#[allow(dead_code)]
impl Where {
    pub fn new(combined_by: Combiner) -> Where {
        Where {
            statement: String::new(),
            combined_by,
        }
    }

    pub fn equal_to(&mut self, field: &str, value: &str) -> &mut Where {
        self.add_combined();

        if value.parse::<f64>().is_ok() {
            self.statement.push_str(&format!(" {} = {} ", field, value));
        } else {
            self.statement
                .push_str(&format!(" {} = '{}' ", field, value));
        }

        self
    }

    pub fn not_equal_to(&mut self, field: &str, value: &str) -> &mut Where {
        self.add_combined();
        if value.parse::<f64>().is_ok() {
            self.statement
                .push_str(&format!(" {} != {} ", field, value));
        } else {
            self.statement
                .push_str(&format!(" {} != '{}' ", field, value));
        }
        self
    }

    pub fn greater_than(&mut self, field: &str, value: &str) -> &mut Where {
        self.add_combined();
        if value.parse::<f64>().is_ok() {
            self.statement.push_str(&format!(" {} > {} ", field, value));
        } else {
            self.statement
                .push_str(&format!(" {} > '{}' ", field, value));
        }
        self
    }

    pub fn greater_than_equal(&mut self, field: &str, value: &str) -> &mut Where {
        self.add_combined();
        if value.parse::<f64>().is_ok() {
            self.statement.push_str(&format!(" {} >= {} ", field, value));
        } else {
            self.statement
                .push_str(&format!(" {} >= '{}' ", field, value));
        }
        self
    }

    pub fn build(&self) -> String {
        if self.statement.len() > 0 {
            return "WHERE".to_string() + &self.statement;
        }
        "".to_string()
    }

    fn add_combined(&mut self) {
        let combined = match self.combined_by {
            Combiner::AND => "AND",
            Combiner::OR => "OR",
        };

        if self.statement.len() > 0 {
            self.statement.push_str(&format!("{}", combined))
        }
    }
}

#[allow(dead_code)]
pub enum Combiner {
    AND,
    OR,
}
