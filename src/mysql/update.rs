use super::where_::{Combiner, Where};

pub struct Update<'a> {
    table: &'a str,
    set: Vec<[&'a str; 2]>,
    where_: Where,
}

#[allow(dead_code)]
impl<'a> Update<'a> {
    pub fn new(table: &'a str) -> Update<'a> {
        Update {
            table,
            set: vec![],
            where_: Where::new(Combiner::And),
        }
    }

    pub fn set(&mut self, set: Vec<[&'a str; 2]>) -> &mut Update<'a> {
        self.set = set;
        self
    }

    pub fn where_(&mut self, where_: Where) -> &mut Update<'a> {
        self.where_ = where_;
        self
    }

    pub fn build(&self) -> String {
        let mut statement = format!("UPDATE {} SET", self.table);

        for s in &self.set {
            if s[1].parse::<f64>().is_ok() {
                statement.push_str(&format!(" {} = {},", s[0], s[1]));
            } else {
                statement.push_str(&format!(" {} = '{}',", s[0], s[1]));
            }
        }
        statement.pop();

        statement.push_str(&format!(" {}", &self.where_.build()));

        statement = statement.trim().to_string() + ";";
        statement
    }
}
