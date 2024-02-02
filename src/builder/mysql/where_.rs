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
        self.add_comparative_predicate("=", field, value)
    }

    pub fn not_equal_to(&mut self, field: &str, value: &str) -> &mut Where {
        self.add_comparative_predicate("!=", field, value)
    }

    pub fn greater_than(&mut self, field: &str, value: &str) -> &mut Where {
        self.add_comparative_predicate(">", field, value)
    }

    pub fn greater_than_equal(&mut self, field: &str, value: &str) -> &mut Where {
        self.add_comparative_predicate(">=", field, value)
    }

    pub fn less_than(&mut self, field: &str, value: &str) -> &mut Where {
        self.add_comparative_predicate("<", field, value)
    }

    pub fn less_than_equal(&mut self, field: &str, value: &str) -> &mut Where {
        self.add_comparative_predicate("<=", field, value)
    }

    pub fn is_null(&mut self, field: &str) -> &mut Where {
        self.add_self_comparative_predicate("ISNULL", field);
        self
    }

    pub fn is_not_null(&mut self, field: &str) -> &mut Where {
        self.add_self_comparative_predicate("IS NOT NULL", field);
        self
    }

    pub fn build(&self) -> String {
        if self.statement.len() > 0 {
            return "WHERE".to_string() + &self.statement;
        }
        "".to_string()
    }

    fn add_combiner(&mut self) {
        let combined = match self.combined_by {
            Combiner::AND => "AND",
            Combiner::OR => "OR",
        };

        if self.statement.len() > 0 {
            self.statement.push_str(&format!("{}", combined))
        }
    }

    fn add_comparative_predicate(
        &mut self,
        operator: &str,
        field: &str,
        value: &str,
    ) -> &mut Where {
        self.add_combiner();
        if value.parse::<f64>().is_ok() {
            self.statement
                .push_str(&format!(" {} {} {} ", field, operator, value));
        } else {
            self.statement
                .push_str(&format!(" {} {} '{}' ", field, operator, value));
        }
        self
    }

    fn add_self_comparative_predicate(&mut self, operator: &str, field: &str) -> &mut Where {
        self.add_combiner();
        self.statement
            .push_str(&format!(" {} {} ", field, operator));
        self
    }
}

#[allow(dead_code)]
pub enum Combiner {
    AND,
    OR,
}
