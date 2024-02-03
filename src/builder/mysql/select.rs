use super::where_::{Combiner, Where};

pub struct Select {
    distinct: bool,
    from: String,
    limit: u32,
    offset: u32,
    columns: String,
    where_: Where,
    group: String,
    order: String,
}

#[allow(dead_code)]
impl Select {
    pub fn new() -> Select {
        Select {
            distinct: false,
            from: String::new(),
            limit: 0,
            offset: 0,
            columns: String::new(),
            where_: Where::new(Combiner::AND),
            group: String::new(),
            order: String::new(),
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

    pub fn columns(&mut self, columns: &str) -> &mut Select {
        self.columns = columns.to_string();
        self
    }

    pub fn where_(&mut self, where_: Where) -> &mut Select {
        self.where_ = where_;
        self
    }

    pub fn group(&mut self, group: &str) -> &mut Select {
        self.group = group.to_string();
        self
    }

    pub fn order(&mut self, order: &str) -> &mut Select {
        self.order = order.to_string();
        self
    }

    pub fn limit(&mut self, limit: u32) {
        self.limit = limit;
    }

    pub fn offset(&mut self, offset: u32) {
        self.offset = offset;
    }

    pub fn build(&self) -> String {
        let mut statement = "SELECT ".to_string();

        if self.distinct {
            statement.push_str("DISTINCT ");
        }

        if self.columns.len() > 0 {
            statement.push_str(&format!("{} ", self.columns.trim_end()));
        } else {
            statement.push_str("* ");
        }

        if self.from.len() > 0 {
            statement.push_str(&format!("FROM {} ", self.from));
        }

        statement.push_str(&self.where_.build());

        if self.group.len() > 0 {
            statement.push_str("GROUP BY ");
            statement.push_str(&format!("{} ", self.group.trim_end()));
        }

        if self.order.len() > 0 {
            statement.push_str("ORDER BY ");
            statement.push_str(&format!("{} ", self.order.trim_end()));
        }

        if self.limit > 0 {
            statement.push_str(&format!("LIMIT {}", self.limit));
        }

        if self.offset > 0 && self.limit > 0 {
            statement.push_str(&format!("OFFSET {}", self.offset));
        }

        statement = statement.trim().to_string() + ";";
        statement
    }
}
