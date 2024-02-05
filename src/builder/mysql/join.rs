#[allow(dead_code)]
pub struct Join {
    table: String,
    join_type: JoinType,
    columns: String,
    on: String,
}

#[allow(dead_code)]

impl Join {
    pub fn new(table: String, on: String, columns: String, join_type: JoinType) -> Join {
        Join {
            table,
            on,
            columns,
            join_type,
        }
    }

    pub fn build(&self) -> String {
        let join_type = match self.join_type {
            JoinType::Inner => "INNER",
            JoinType::Left => "LEFT",
            JoinType::Right => "RIGHT",
            JoinType::Full => "FULL",
            JoinType::LeftOuter => "LEFT OUTER",
            JoinType::RightOuter => "RIGHT OUTER",
        };

        let statement = format!("{} JOIN {} ON {} ", join_type, self.table, self.on);
        statement
    }
}

#[allow(dead_code)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    RightOuter,
    LeftOuter,
    Full,
}
