pub struct Select {
    table: String,
    distinct: bool,
    columns: Option<String>,
    group: Option<String>,
    order: Option<String>,
    limit: Option<u32>,
    offset: Option<u32>,
}

impl Select {
    pub fn new<T: Into<String>>(table: T) -> Select {
        Select {
            table: table.into(),
            distinct: false,
            columns: None,
            group: None,
            order: None,
            limit: None,
            offset: None,
        }
    }

    pub fn distinct(&mut self) -> &mut Self {
        self.distinct = true;
        self
    }

    pub fn columns<T: Into<String>>(&mut self, columns: T) -> &mut Self {
        self.columns = Some(columns.into());
        self
    }

    pub fn group<T: Into<String>>(&mut self, group: T) -> &mut Self {
        self.group = Some(group.into());
        self
    }

    pub fn order<T: Into<String>>(&mut self, order: T) -> &mut Self {
        self.order = Some(order.into());
        self
    }

    pub fn limit(&mut self, limit: u32) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(&mut self, offset: u32) -> &mut Self {
        self.offset = Some(offset);
        self
    }

    pub fn build(&self) -> String {
        let mut statement = String::from("SELECT");

        if self.distinct {
            statement.push_str(" DISTINCT");
        }

        if let Some(columns) = &self.columns {
            statement.push_str(&format!(" {}", columns));
        } else {
            statement.push_str(" *");
        }

        statement.push_str(&format!(" FROM {}", self.table));

        if let Some(group) = &self.group {
            statement.push_str(&format!(" GROUP BY {}", group));
        }

        if let Some(order) = &self.order {
            statement.push_str(&format!(" ORDER BY {}", order));
        }

        if let Some(limit) = &self.limit {
            statement.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = &self.offset {
            statement.push_str(&format!(" OFFSET {}", offset));
        }

        statement.push(';');
        statement
    }
}
