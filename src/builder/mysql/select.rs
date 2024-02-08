use super::{
    join::{Join, JoinType},
    where_::{Combiner, Where},
};

pub struct Select<'a> {
    distinct: bool,
    from: &'a str,
    limit: u32,
    offset: u32,
    columns: &'a str,
    where_: Where,
    group: &'a str,
    order: &'a str,

    joins: Vec<Join>,
}

#[allow(dead_code)]
impl<'a> Select<'a> {
    pub fn new() -> Select<'a> {
        Select {
            distinct: false,
            from: "",
            limit: 0,
            offset: 0,
            columns: "",
            where_: Where::new(Combiner::And),
            group: "",
            order: "",
            joins: vec![],
        }
    }

    pub fn distinct(&mut self) -> &mut Select<'a> {
        self.distinct = true;
        self
    }

    pub fn from(&mut self, from: &'a str) -> &mut Select<'a> {
        self.from = from;
        self
    }

    pub fn columns(&mut self, columns: &'a str) -> &mut Select<'a> {
        self.columns = columns;
        self
    }

    pub fn where_(&mut self, where_: Where) -> &mut Select<'a> {
        self.where_ = where_;
        self
    }

    pub fn join(
        &mut self,
        table: &str,
        on: &str,
        join_type: JoinType,
    ) -> &mut Select<'a> {
        self.joins.push(Join::new(
            table.to_string(),
            on.to_string(),
            join_type,
        ));
        self
    }

    pub fn group(&mut self, group: &'a str) -> &mut Select<'a> {
        self.group = group;
        self
    }

    pub fn order(&mut self, order: &'a str) -> &mut Select<'a> {
        self.order = order;
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

        if self.joins.len() > 0 {
            for join in &self.joins {
                statement.push_str(&join.build());
            }
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
