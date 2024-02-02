#[allow(dead_code)]
pub struct Where {
    statement: String,
    combined_by: Operator,
}

#[allow(dead_code)]
impl Where {
    pub fn new(combined_by: Operator) -> Where {
        Where {
            statement: String::new(),
            combined_by,
        }
    }

    pub fn equal_to(&mut self, field: &str, value: &str) -> &mut Where {
        self.statement.push_str(&format!(" {} = {} ", field, value));
        // self.add_combined();
        self
    }

    pub fn build(&self) -> String {
        if self.statement.len() > 0 {
            return "WHERE ".to_string() + &self.statement;
        }
        "".to_string()
    }

    // fn add_combined(&mut self) {
    //     let mut combined = "";
    //     self.statement.push_str(&format!(" {} ",))
    // }
}

#[allow(dead_code)]
pub enum Operator {
    AND,
    OR,
}
