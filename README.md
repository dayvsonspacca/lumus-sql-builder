# Lumus sql builder
This repository contains a project written in Rust with the aim of building SQL queries. It was created as part of an effort to learn more about the Rust programming language and its applications.

## About project

The project is an SQL query builder that allows you to create complex SQL queries programmatically and intuitively. It supports a variety of SQL operations, including column selection, joins, WHERE clauses, grouping, sorting, and much more.

Here is an example of how the code can be used to build an SQL query:

```rust
use lumus_sql_builder::sqlite::{CreateTable, Column};

fn main() {
    let create_table = CreateTable::new("employees", vec![
        Column::new("id").integer().primary_key().auto_increment(),
        Column::new("name").text().not_null(),
        Column::new("age").integer().not_null(),
        Column::new("department").text().default("'Undefined'"),
        Column::new("salary").real(),
        Column::new("hired_date").datetime(),
        Column::new("manager_id").integer()
    ]);

    println!("{}", create_table.build());
}

```
# Output: 
```sql
CREATE TABLE employees (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    age INTEGER NOT NULL,
    department TEXT DEFAULT 'Undefined',
    salary REAL
    hired_date DATETIME,
    manager_id INTEGER
);
```
