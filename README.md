# Lumus sql builder
This repository contains a project written in Rust with the aim of building SQL queries. It was created as part of an effort to learn more about the Rust programming language and its applications.

## About project

The project is an SQL query builder that allows you to create complex SQL queries programmatically and intuitively. It supports a variety of SQL operations, including column selection, joins, WHERE clauses, grouping, sorting, and much more.

Here is an example of how the code can be used to build an SQL query:

```rust
use lumus_sql_builder::mysql::{
    join::JoinType,
    select::Select,
    where_::{Combiner, Where},
};

pub fn main() {
    let mut select = Select::new();
    let mut where_ = Where::new(Combiner::And);
    where_
        .not_equal_to("email", "spacca.dayvson@gmail.com")
        .greater_than_equal("age", "2")
        .less_than("salary", "230.00")
        .less_than_equal("age", "25")
        .is_null("genre");

    select
        .columns("name, age, email, salary")
        .from("users_tb u")
        .join("emails_tb e", "e.user_id = u.user_id", JoinType::Left)
        .join("phones_tb p", "p.user_id = u.user_id", JoinType::Left)
        .where_(where_);

    println!("{}", select.build());
}

```
# Output: 
```sql
SELECT name, age, email, salary FROM users_tb u LEFT JOIN emails_tb e ON e.user_id = u.user_id LEFT JOIN phones_tb p ON p.user_id = u.user_id WHERE email != 'spacca.dayvson@gmail.com' AND age >= 2 AND salary < 230.00 AND age <= 25 AND genre ISNULL;
```
