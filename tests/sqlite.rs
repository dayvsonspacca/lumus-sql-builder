use lumus_sql_builder::sqlite::{Column, CreateTable, Delete, Insert, Select, Update, Where};

#[test]
fn test_columns() {
    let col = Column::new("id")
        .integer()
        .auto_increment()
        .primary_key()
        .build()
        .unwrap();
    assert_eq!(col, "id INTEGER AUTOINCREMENT PRIMARY KEY");

    let col = Column::new("name")
        .text()
        .unique()
        .not_null()
        .build()
        .unwrap();
    assert_eq!(col, "name TEXT UNIQUE NOT NULL");

    let col = Column::new("price").real().not_null().build().unwrap();
    assert_eq!(col, "price REAL NOT NULL");

    let col = Column::new("is_active")
        .boolean()
        .default("1")
        .build()
        .unwrap();
    assert_eq!(col, "is_active BOOLEAN DEFAULT 1");

    let col = Column::new("data").blob().unique().build().unwrap();
    assert_eq!(col, "data BLOB UNIQUE");

    let col = Column::new("balance").numeric().not_null().build().unwrap();
    assert_eq!(col, "balance NUMERIC NOT NULL");

    let col = Column::new("created_at")
        .date()
        .default("CURRENT_DATE")
        .build()
        .unwrap();
    assert_eq!(col, "created_at DATE DEFAULT CURRENT_DATE");

    let col = Column::new("start_time").time().not_null().build().unwrap();
    assert_eq!(col, "start_time TIME NOT NULL");

    let col = Column::new("updated_at")
        .datetime()
        .primary_key()
        .build()
        .unwrap();
    assert_eq!(col, "updated_at DATETIME PRIMARY KEY");

    let result = Column::new("invalid").build();
    assert!(result.is_err());
}

#[test]
fn test_create_table() {
    let create_table = CreateTable::new("users", vec![Column::new("name").text().not_null()])
        .build()
        .unwrap();
    assert_eq!(create_table, "CREATE TABLE users (name TEXT NOT NULL);");

    let create_table = CreateTable::new(
        "users",
        vec![
            Column::new("id").integer().primary_key().auto_increment(),
            Column::new("name").text().not_null(),
            Column::new("email").text().unique(),
        ],
    )
    .build()
    .unwrap();
    assert_eq!(
        create_table,
        "CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL, email TEXT UNIQUE);"
    );

    let create_table = CreateTable::new("users", vec![Column::new("name").text().not_null()])
        .if_not_exists()
        .build()
        .unwrap();
    assert_eq!(
        create_table,
        "CREATE TABLE IF NOT EXISTS users (name TEXT NOT NULL);"
    );

    let result = CreateTable::new("users", vec![]).build();
    assert!(result.is_err());

    let result = CreateTable::new("", vec![Column::new("name").text()]).build();
    assert!(result.is_err());

    let create_table = CreateTable::new(
        "orders",
        vec![
            Column::new("id").integer().primary_key().auto_increment(),
            Column::new("total").numeric().default("0"),
        ],
    )
    .build()
    .unwrap();
    assert_eq!(
        create_table,
        "CREATE TABLE orders (id INTEGER PRIMARY KEY AUTOINCREMENT, total NUMERIC DEFAULT 0);"
    );

    let create_table = CreateTable::new(
        "events",
        vec![
            Column::new("event_date").date().not_null(),
            Column::new("event_time").time().not_null(),
        ],
    )
    .build()
    .unwrap();
    assert_eq!(
        create_table,
        "CREATE TABLE events (event_date DATE NOT NULL, event_time TIME NOT NULL);"
    );

    let create_table = CreateTable::new(
        "files",
        vec![
            Column::new("file_data").blob().not_null(),
            Column::new("is_active").boolean().default("1"),
        ],
    )
    .build()
    .unwrap();
    assert_eq!(
        create_table,
        "CREATE TABLE files (file_data BLOB NOT NULL, is_active BOOLEAN DEFAULT 1);"
    );

    let create_table = CreateTable::new(
        "categories",
        vec![Column::new("category_name").text().primary_key()],
    )
    .build()
    .unwrap();
    assert_eq!(
        create_table,
        "CREATE TABLE categories (category_name TEXT PRIMARY KEY);"
    );

    let create_table = CreateTable::new(
        "users",
        vec![
            Column::new("username").text().not_null().unique(),
            Column::new("email").text().not_null().unique(),
        ],
    )
    .if_not_exists()
    .build()
    .unwrap();
    assert_eq!(
        create_table,
        "CREATE TABLE IF NOT EXISTS users (username TEXT NOT NULL UNIQUE, email TEXT NOT NULL UNIQUE);"
    );
}

#[test]
fn test_select_queries() {
    let select = Select::new("users").build().unwrap();
    assert_eq!(select, "SELECT * FROM users;");

    let select = Select::new("users").columns("name, age").build().unwrap();
    assert_eq!(select, "SELECT name, age FROM users;");

    let mut select = Select::new("users");
    select.distinct().columns("name");
    let result = select.build().unwrap();
    assert_eq!(result, "SELECT DISTINCT name FROM users;");

    let select = Select::new("users")
        .condition("age > 18".to_string())
        .build()
        .unwrap();
    assert_eq!(select, "SELECT * FROM users WHERE age > 18;");

    let select = Select::new("users")
        .columns("name, age")
        .condition("age > 18".to_string())
        .build()
        .unwrap();
    assert_eq!(select, "SELECT name, age FROM users WHERE age > 18;");

    let select = Select::new("users").group("city").build().unwrap();
    assert_eq!(select, "SELECT * FROM users GROUP BY city;");

    let select = Select::new("users").order("name").build().unwrap();
    assert_eq!(select, "SELECT * FROM users ORDER BY name;");

    let select = Select::new("users").limit(10).build().unwrap();
    assert_eq!(select, "SELECT * FROM users LIMIT 10;");

    let select = Select::new("users").offset(5).build().unwrap();
    assert_eq!(select, "SELECT * FROM users OFFSET 5;");

    let select = Select::new("users")
        .columns("name, age")
        .condition("age > 18".to_string())
        .group("city")
        .order("name")
        .limit(10)
        .offset(5)
        .build()
        .unwrap();
    assert_eq!(
        select,
        "SELECT name, age FROM users WHERE age > 18 GROUP BY city ORDER BY name LIMIT 10 OFFSET 5;"
    );
}

#[test]
fn test_insert_queries() {
    let insert = Insert::new("users")
        .values(vec![("name", "João"), ("age", "30")])
        .build()
        .unwrap();
    assert_eq!(
        insert,
        "INSERT INTO users (name, age) VALUES ('João', '30');"
    );

    let insert = Insert::new("users")
        .values(vec![("name", "João")])
        .build()
        .unwrap();
    assert_eq!(insert, "INSERT INTO users (name) VALUES ('João');");

    let insert = Insert::new("users")
        .values(vec![("name", ""), ("age", "30")])
        .build();
    assert!(insert.is_err());

    let insert = Insert::new("users")
        .values(vec![("", "João"), ("age", "30")])
        .build();
    assert!(insert.is_err());

    let insert = Insert::new("users")
        .values(vec![("name", "João"), ("age", "")])
        .build();
    assert!(insert.is_err());

    let insert = Insert::new("users").values(vec![("", "")]).build();
    assert!(insert.is_err());

    let insert = Insert::new("users").build();
    assert!(insert.is_err());

    let insert = Insert::new("")
        .values(vec![("name", "João"), ("age", "30")])
        .build();
    assert!(insert.is_err());

    let insert = Insert::new("users")
        .values(vec![
            ("name", "M' Carlos"),
            ("age", "30"),
            ("city", "São Paulo"),
        ])
        .build()
        .unwrap();
    assert_eq!(
        insert,
        "INSERT INTO users (name, age, city) VALUES (\'M' Carlos\', '30', 'São Paulo');"
    );

    let insert = Insert::new("users")
        .values(vec![("name", "João"), ("department", "TI")])
        .build()
        .unwrap();
    assert_eq!(
        insert,
        "INSERT INTO users (name, department) VALUES ('João', 'TI');"
    );
}

#[test]
fn test_where_conditions() {
    let mut condition = Where::new();
    condition.equal_to("name", "Dayvson Spacca");
    assert_eq!(condition.build(), "name = 'Dayvson Spacca'");

    let mut condition = Where::new();
    condition
        .not_equal_to("age", "30")
        .and()
        .equal_to("status", "active");
    assert_eq!(condition.build(), "age != '30' AND status = 'active'");

    let mut condition = Where::new();
    condition
        .greater_than("age", "21")
        .or()
        .less_than("age", "18");
    assert_eq!(condition.build(), "age > '21' OR age < '18'");

    let mut condition = Where::new();
    condition
        .greater_than_equal("salary", "5000")
        .and()
        .less_than_equal("salary", "10000");
    assert_eq!(condition.build(), "salary >= '5000' AND salary <= '10000'");

    let mut condition = Where::new();
    condition
        .is_null("manager_id")
        .and()
        .is_not_null("department");
    assert_eq!(
        condition.build(),
        "manager_id IS NULL AND department IS NOT NULL"
    );

    let mut condition = Where::new();
    condition
        .inside("department", vec!["IT", "HR", "Finance"])
        .and()
        .not_inside("location", vec!["Remote", "Offsite"]);
    assert_eq!(
        condition.build(),
        "department IN ('IT', 'HR', 'Finance') AND location NOT IN ('Remote', 'Offsite')"
    );

    let mut condition = Where::new();
    condition
        .like("name", "%Spacca%")
        .or()
        .not_like("email", "%@example.com");
    assert_eq!(
        condition.build(),
        "name LIKE '%Spacca%' OR email NOT LIKE '%@example.com'"
    );

    let mut condition = Where::new();
    condition
        .nest()
        .equal_to("status", "active")
        .or()
        .equal_to("status", "pending")
        .unnest()
        .and()
        .greater_than("created_at", "2024-01-01");
    assert_eq!(
        condition.build(),
        "(status = 'active' OR status = 'pending') AND created_at > '2024-01-01'"
    );

    let mut condition = Where::from("role = 'admin'");
    condition
        .and()
        .like("name", "%Spacca%")
        .or()
        .not_equal_to("department", "HR")
        .and()
        .is_null("manager_id");
    assert_eq!(
        condition.build(),
        "role = 'admin' AND name LIKE '%Spacca%' OR department != 'HR' AND manager_id IS NULL"
    );

    let mut condition = Where::new();
    condition
        .equal_to("name", "Dayvson Spacca")
        .and()
        .greater_than("age", "21")
        .or()
        .inside("department", vec!["IT", "HR"])
        .and()
        .like("email", "%@company.com")
        .and()
        .is_not_null("manager_id")
        .or()
        .not_like("role", "intern")
        .and()
        .nest()
        .not_inside("location", vec!["Remote", "O'nsite"])
        .unnest();
    assert_eq!(
        condition.build(),
        "name = 'Dayvson Spacca' AND age > '21' OR department IN ('IT', 'HR') AND email LIKE '%@company.com' AND manager_id IS NOT NULL OR role NOT LIKE 'intern' AND (location NOT IN ('Remote', 'O'nsite'))"
    );
}

#[test]
fn test_update_clause() {
    let update = Update::new("users_tb")
        .set(vec![("name", "João")])
        .build()
        .unwrap();
    assert_eq!(update, "UPDATE users_tb SET name = 'João';");

    let update = Update::new("products_tb")
        .set(vec![("price", "9.99"), ("stock", "100")])
        .build()
        .unwrap();
    assert_eq!(
        update,
        "UPDATE products_tb SET price = '9.99', stock = '100';"
    );

    let mut condition = Where::new();
    condition.equal_to("age", "21");

    let update = Update::new("users_tb")
        .set(vec![("name", "João")])
        .condition(condition.build())
        .build()
        .unwrap();
    assert_eq!(
        update,
        "UPDATE users_tb SET name = 'João' WHERE age = '21';"
    );

    let update_result = Update::new("users_tb").build();
    assert!(update_result.is_err());
    assert_eq!(
        format!("{}", update_result.unwrap_err()),
        "The column and the value to be inserted cannot be empty."
    );

    let update_result = Update::new("").set(vec![("name", "João")]).build();
    assert!(update_result.is_err());
    assert_eq!(
        format!("{}", update_result.unwrap_err()),
        "Table name cannot be empty."
    );

    let update_result = Update::new("users_tb").set(vec![("", "João")]).build();
    assert!(update_result.is_err());
    assert_eq!(
        format!("{}", update_result.unwrap_err()),
        "Column name cannot be empty."
    );

    let update_result = Update::new("users_tb").set(vec![("name", "")]).build();
    assert!(update_result.is_err());
    assert_eq!(
        format!("{}", update_result.unwrap_err()),
        "The value cannot be empty."
    );

    let mut condition = Where::new();
    condition
        .equal_to("age", "21")
        .and()
        .greater_than("score", "80");

    let update = Update::new("users_tb")
        .set(vec![("status", "premium")])
        .condition(condition.build())
        .build()
        .unwrap();
    assert_eq!(
        update,
        "UPDATE users_tb SET status = 'premium' WHERE age = '21' AND score > '80';"
    );

    let update = Update::new("users_tb")
        .set(vec![("status", "active")])
        .build()
        .unwrap();
    assert_eq!(update, "UPDATE users_tb SET status = 'active';");

    let update = Update::new("users_tb")
        .set(vec![("name", "O'Reilly")])
        .build()
        .unwrap();
    assert_eq!(update, "UPDATE users_tb SET name = \'O'Reilly\';");
}

#[test]
fn test_delete_clause() {
    let delete = Delete::new("users_tb").build().unwrap();
    assert_eq!(delete, "DELETE FROM users_tb;");

    let mut condition = Where::new();
    condition.equal_to("age", "21");

    let delete = Delete::new("users_tb")
        .condition(condition.build())
        .build()
        .unwrap();
    assert_eq!(delete, "DELETE FROM users_tb WHERE age = '21';");

    let delete_result = Delete::new("").build();
    assert!(delete_result.is_err());
    assert_eq!(
        format!("{}", delete_result.unwrap_err()),
        "Table name cannot be empty."
    );

    let mut condition = Where::new();
    condition
        .equal_to("age", "21")
        .and()
        .greater_than("score", "80");

    let delete = Delete::new("users_tb")
        .condition(condition.build())
        .build()
        .unwrap();
    assert_eq!(
        delete,
        "DELETE FROM users_tb WHERE age = '21' AND score > '80';"
    );

    let delete_result = Delete::new("users_tb").condition("".to_string()).build();
    assert!(delete_result.is_err());
    assert_eq!(
        format!("{}", delete_result.unwrap_err()),
        "The conditions cannot be empty."
    );

    let delete = Delete::new("users_tb").build().unwrap();

    assert_eq!(delete, "DELETE FROM users_tb;");
}
