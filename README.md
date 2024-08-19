# Lumus sql builder

## About project

Lumus SQL Builder is a Rust library that allows you to programmatically and intuitively construct complex SQL queries for simple projects. It supports a variety of SQL operations, including column selection, joins, WHERE clauses, grouping, sorting, and more.

## Features

-   Table creation
-   Data insertion
-   Data selection with support for DISTINCT, GROUP BY, ORDER BY, LIMIT, and OFFSET
-   A simple way to make WHERE clauses

## Example Usage

### Creating a Table

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

    println!("{}", create_table.build().unwrap());
}
```

### Output

```sql
CREATE TABLE employees (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    age INTEGER NOT NULL,
    department TEXT DEFAULT 'Undefined',
    salary REAL,
    hired_date DATETIME,
    manager_id INTEGER
);
```

### Inserting Data

```rust
use lumus_sql_builder::sqlite::Insert;

fn main() {
    let insert = Insert::new("employees").values(vec![
        ("name", "John"),
        ("age", "30"),
        ("department", "IT"),
        ("salary", "5000.00"),
        ("hired_date", "2024-03-20"),
        ("manager_id", "1")
    ]);

    println!("{}", insert.build().unwrap());
}
```

### Output

```sql
INSERT INTO employees (name, age, department, salary, hired_date, manager_id) VALUES ('John', '30', 'IT', '5000.00', '2024-03-20', '1');
```

### Selecting Data

```rust
use lumus_sql_builder::sqlite::{Select, Where};

fn main() {
    let select = Select::new("employees")
        .columns("name, age, department")
        .condition(Where::from("age > 25").build())
        .order("age DESC")
        .limit(10);

    println!("{}", select.build().unwrap());
}
```

### Output

```sql
SELECT name, age, department FROM employees WHERE age > 25 ORDER BY age DESC LIMIT 10;
```
