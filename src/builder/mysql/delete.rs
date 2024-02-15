use super::where_::{Combiner, Where};

pub struct Delete<'a> {
    table: &'a str,
    where_: Where,
}

#[allow(dead_code)]
impl<'a> Delete<'a> {
    pub fn new(table: &'a str) -> Delete<'a> {
        Delete {
            table,
            where_: Where::new(Combiner::And),
        }
    }

    pub fn where_(&mut self, where_: Where) -> &mut Delete<'a> {
        self.where_ = where_;
        self
    }

    pub fn build(&self) -> String {
        let mut statement = format!("DELETE FROM {}", self.table);

        statement.push_str(&format!(" {}", &self.where_.build()));

        statement = statement.trim().to_string() + ";";
        statement
    }
}
